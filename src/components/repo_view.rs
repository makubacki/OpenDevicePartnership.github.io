use leptos::*;
use leptos::html::*;
use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::window;

// Fix the deprecated JsStatic and naming convention
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "d3"], thread_local_v2)]
    static D3: JsValue;
}

#[component]
pub fn RepositoryGraph(
    #[prop(into)] nodes: String,
    #[prop(into)] links: String,
) -> impl IntoView {
    let (nodes_data, _) = signal(nodes);
    let (links_data, _) = signal(links);

    // Track initialization to prevent multiple runs
    let (initialized, set_initialized) = signal(false);

    // Use Effect::new for initialization
    Effect::new(move |_| {
        if !initialized.get() {
            let nodes_str = nodes_data.get_untracked();
            let links_str = links_data.get_untracked();
            
            // Initialize D3 graph when component mounts
            init_d3_graph(&nodes_str, &links_str);
            
            // Inject CSS styles
            inject_styles();
            
            set_initialized.set(true);
        }
    });

    view! {
        <div class="repository-graph">
            <div id="zoom-controls">
                <button id="zoom-in">"+"</button>
                <button id="zoom-out">"−"</button>
                <button id="zoom-fit">"⛶"</button>
            </div>
            <svg width="100%" height="100%" style="position:absolute;"></svg>
        </div>
    }
}

// Function to inject CSS styles
fn inject_styles() {
    let window = window().unwrap();
    let document = window.document().unwrap();
    let head = document.head().unwrap();
    
    let style_element = document.create_element("style").unwrap();
    style_element.set_inner_html(repository_graph_styles());
    head.append_child(&style_element).unwrap();
}

#[wasm_bindgen]
pub fn init_d3_graph(nodes_json: &str, links_json: &str) {
    let window = window().unwrap();
    let document = window.document().unwrap();
    
    // Store data globally for D3 to access
    let js_code = format!(r##"
        window.graphNodes = {};
        window.graphLinks = {};
        
        window.initGraph = function() {{
            const nodes = window.graphNodes;
            const links = window.graphLinks;

            const width = window.innerWidth;
            const height = window.innerHeight;
            const spacing = 300;

            const svg = d3.select("svg").attr("viewBox", [0, 0, width, height]);
            const zoomLayer = svg.append("g").attr("class", "zoom-layer");

            const grouped = {{}};
            const boxHeight = 60;
            const verticalGap = 20;

            nodes.forEach(d => {{
                d.fx = d.order * spacing;
                if (!grouped[d.order]) grouped[d.order] = [];
                grouped[d.order].push(d);
            }});

            Object.values(grouped).forEach(group => {{
                const totalHeight = group.length * (boxHeight + verticalGap);
                const startY = (height - totalHeight) / 2;
                group.forEach((node, i) => {{
                    node.fy = startY + i * (boxHeight + verticalGap);
                    node.initialFy = node.fy;
                }});
            }});

            const classifications = Array.from(
                new Set(nodes.map(d => JSON.stringify({{ classification: d.classification, order: d.order }})))
            ).map(s => JSON.parse(s))
             .sort((a, b) => a.order - b.order);

            const headerGroup = zoomLayer.append("g").attr("class", "column-headers");

            classifications.forEach(({{
                classification, order
            }}) => {{
                const x = order * spacing;

                headerGroup.append("line")
                    .attr("x1", x)
                    .attr("y1", 0)
                    .attr("x2", x)
                    .attr("y2", height)
                    .attr("stroke", "#ccc")
                    .attr("stroke-width", 2)
                    .attr("stroke-dasharray", "4,4");

                headerGroup.append("text")
                    .attr("x", x)
                    .attr("y", 30)
                    .attr("text-anchor", "middle")
                    .attr("fill", "#666")
                    .attr("font-size", "16px")
                    .attr("font-weight", "bold")
                    .text(classification);
            }});

            const link = zoomLayer.append("g")
                .selectAll("path")
                .data(links)
                .join("path")
                .attr("class", "link");

            const simulation = d3.forceSimulation(nodes)
                .force("link", d3.forceLink(links).id(d => d.id).distance(150))
                .force("charge", d3.forceManyBody().strength(-300));

            const node = zoomLayer.append("g")
                .selectAll("g")
                .data(nodes)
                .join("g")
                .attr("class", "node")
                .call(drag(simulation));

            node.append("text")
                .attr("text-anchor", "middle")
                .attr("dy", "0.35em")
                .text(d => d.name);

            node.each(function(d) {{
                const textEl = d3.select(this).select("text").node();
                const textWidth = textEl.getComputedTextLength();
                d.boxWidth = textWidth + 20;

                const g = d3.select(this);
                const rect = g.insert("rect", "text")
                    .attr("x", -d.boxWidth / 2)
                    .attr("y", -20)
                    .attr("width", d.boxWidth)
                    .attr("height", 40)
                    .attr("rx", 10)
                    .attr("ry", 10)
                    .attr("fill", "white")
                    .attr("stroke", "#333")
                    .attr("stroke-width", 1.5);

                d._rect = rect;
            }});

            node
                .on("mouseover", function(event, d) {{
                    d._rect.attr("fill", "#9BFABE");
                }})
                .on("mouseout", function(event, d) {{
                    d._rect.attr("fill", "white");
                }})
                .on("click", function(event, d) {{
                    window.open(d.url, "_blank");
                }});

            const zoomMin = 0.1;
            const zoomMax = 3;

            let currentTransform = d3.zoomIdentity;

            const zoomBehavior = d3.zoom()
                .scaleExtent([zoomMin, zoomMax])
                .filter((event) => {{
                    return event.type !== 'wheel';
                }})
                .on("zoom", (event) => {{
                    currentTransform = event.transform;
                    zoomLayer.attr("transform", currentTransform);
                }});

            svg.call(zoomBehavior);

            function applyZoom() {{
                svg.transition().duration(300)
                   .call(zoomBehavior.transform, currentTransform);
            }}

            function fitToScreen() {{
                const xs = nodes.map(n => n.x);
                const ys = nodes.map(n => n.y);
                const minX = Math.min(...xs);
                const maxX = Math.max(...xs);
                const minY = Math.min(...ys);
                const maxY = Math.max(...ys);

                const boundsWidth = maxX - minX + 200;
                const boundsHeight = maxY - minY + 200;

                const scaleX = width / boundsWidth;
                const scaleY = height / boundsHeight;
                const scale = Math.min(scaleX, scaleY, zoomMax);

                const centerX = (minX + maxX) / 2;
                const centerY = (minY + maxY) / 2;

                const tx = width / 2 - scale * centerX;
                const ty = height / 2 - scale * centerY;

                currentTransform = d3.zoomIdentity.translate(tx, ty).scale(scale);
                return currentTransform;
            }}

            // Track if initial fit has been applied
            let initialFitApplied = false;

            simulation.on("tick", () => {{
                link.attr("d", d => {{
                    const dx = d.target.x - d.source.x;
                    const dy = d.target.y - d.source.y;
                    const dr = Math.sqrt(dx * dx + dy * dy) * 1.5;
                    return `M${{d.source.x}},${{d.source.y}} A${{dr}},${{dr}} 0 0,1 ${{d.target.x}},${{d.target.y}}`;
                }});
                node.attr("transform", d => `translate(${{ d.x }},${{ d.y }})`);
                
                // Apply initial fit after a few ticks when positions have stabilized
                if (!initialFitApplied && simulation.alpha() < 0.5) {{
                    initialFitApplied = true;
                    currentTransform = fitToScreen();
                    svg.call(zoomBehavior.transform, currentTransform);
                }}
            }});

            function drag(simulation) {{
                function dragstarted(event, d) {{
                    if (!event.active) simulation.alphaTarget(0.3).restart();
                }}
                function dragged(event, d) {{
                    d.fy = event.y;
                }}
                function dragended(event, d) {{
                    if (!event.active) simulation.alphaTarget(0);
                    d.fy = d.initialFy;
                }}
                return d3.drag().on("start", dragstarted).on("drag", dragged).on("end", dragended);
            }}

            d3.select("#zoom-in").on("click", () => {{
                const newScale = Math.min(currentTransform.k + 0.1, zoomMax);
                currentTransform = d3.zoomIdentity
                    .translate(currentTransform.x, currentTransform.y)
                    .scale(newScale);
                applyZoom();
            }});

            d3.select("#zoom-out").on("click", () => {{
                const newScale = Math.max(currentTransform.k - 0.1, zoomMin);
                currentTransform = d3.zoomIdentity
                    .translate(currentTransform.x, currentTransform.y)
                    .scale(newScale);
                applyZoom();
            }});

            d3.select("#zoom-fit").on("click", () => {{
                currentTransform = fitToScreen();
                applyZoom();
            }});
        }};
        
        // Load D3 and initialize
        if (typeof d3 === 'undefined') {{
            const script = document.createElement('script');
            script.src = 'https://d3js.org/d3.v7.min.js';
            script.onload = () => window.initGraph();
            document.head.appendChild(script);
        }} else {{
            window.initGraph();
        }}
    "##, nodes_json, links_json);
    
    let script_el = document.create_element("script").unwrap();
    script_el.set_inner_html(&js_code);
    document.head().unwrap().append_child(&script_el).unwrap();
}

// CSS styles function
pub fn repository_graph_styles() -> &'static str {
    r#"
        .repository-graph {
            position: relative;
            width: 100%;
            height: 100vh;
            background: #f8f9fa;
        }
        
        #zoom-controls {
            position: absolute;
            top: 20px;
            left: 20px;
            z-index: 1000;
            display: flex;
            gap: 5px;
        }
        
        #zoom-controls button {
            width: 40px;
            height: 40px;
            border: none;
            background: white;
            border-radius: 5px;
            box-shadow: 0 2px 5px rgba(0,0,0,0.2);
            cursor: pointer;
            font-size: 18px;
            display: flex;
            align-items: center;
            justify-content: center;
        }
        
        #zoom-controls button:hover {
            background: #f0f0f0;
        }
        
        .node {
            cursor: pointer;
        }
        
        .link {
            fill: none;
            stroke: #666;
            stroke-width: 2;
        }
        
        .column-headers text {
            font-family: Arial, sans-serif;
        }
    "#
}
