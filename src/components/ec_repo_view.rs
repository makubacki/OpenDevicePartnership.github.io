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
pub fn RepositoryGraph() -> impl IntoView {
    let (nodes_data, _) = signal(r#"[{"id": 0, "name": "soc-embedded-controller", "url": "https://github.com/OpenDevicePartnership/embedded-power-sequence", "classification": "platform", "order": 1}, {"id": 1, "name": "embedded-services", "url": "https://github.com/OpenDevicePartnership/embedded-services", "classification": "services", "order": 2}, {"id": 2, "name": "embedded-usb-pd", "url": "https://github.com/OpenDevicePartnership/embedded-usb-pd", "classification": "subsystem", "order": 3}, {"id": 3, "name": "embedded-power-sequence", "url": "https://github.com/OpenDevicePartnership/embedded-power-sequence", "classification": "subsystem", "order": 3}, {"id": 4, "name": "embedded-batteries", "url": "https://github.com/OpenDevicePartnership/embedded-batteries", "classification": "subsystem", "order": 3}, {"id": 5, "name": "embedded-cfu", "url": "https://github.com/OpenDevicePartnership/embedded-cfu", "classification": "subsystem", "order": 3}, {"id": 6, "name": "embedded-sensors", "url": "https://github.com/OpenDevicePartnership/embedded-sensors", "classification": "subsystem", "order": 3}, {"id": 7, "name": "embedded-fans", "url": "https://github.com/OpenDevicePartnership/embedded-fans", "classification": "subsystem", "order": 3}, {"id": 8, "name": "embedded-keyboard-rs ", "url": "https://github.com/OpenDevicePartnership/embedded-keyboard-rs", "classification": "subsystem", "order": 3}, {"id": 9, "name": "tps6699x", "url": "https://github.com/OpenDevicePartnership/tps6699x", "classification": "drivers", "order": 4}, {"id": 10, "name": "tmp108", "url": "https://github.com/OpenDevicePartnership/tmp108", "classification": "drivers", "order": 4}, {"id": 11, "name": "is31fl3743b", "url": "https://github.com/OpenDevicePartnership/is31fl3743b", "classification": "drivers", "order": 4}, {"id": 12, "name": "pcal6416a", "url": "https://github.com/OpenDevicePartnership/pcal6416a", "classification": "drivers", "order": 4}, {"id": 13, "name": "bq25773", "url": "https://github.com/OpenDevicePartnership/bq25773", "classification": "drivers", "order": 4}, {"id": 14, "name": "bq40z50", "url": "https://github.com/OpenDevicePartnership/bq40z50", "classification": "drivers", "order": 4}, {"id": 15, "name": "bq25770g", "url": "https://github.com/OpenDevicePartnership/bq25770g", "classification": "drivers", "order": 4}, {"id": 16, "name": "embedded-hal", "url": "https://github.com/rust-embedded/embedded-hal", "classification": "embedded HAL", "order": 5}, {"id": 17, "name": "embassy-imxrt", "url": "https://github.com/OpenDevicePartnership/embassy-imxrt", "classification": "MCU core HALs", "order": 6}, {"id": 18, "name": "mimxrt600-fcb", "url": "https://github.com/OpenDevicePartnership/mimxrt600-fcb", "classification": "MCU PACs", "order": 7}, {"id": 19, "name": "mimxrt633s-pac ", "url": "https://github.com/OpenDevicePartnership/mimxrt633s-pac", "classification": "MCU PACs", "order": 7}]"#);
    let (links_data, _) = signal(r#"[{"source": 0, "target": 17}, {"source": 0, "target": 1}, {"source": 1, "target": 2}, {"source": 1, "target": 3}, {"source": 1, "target": 4}, {"source": 1, "target": 5}, {"source": 1, "target": 6}, {"source": 1, "target": 7}, {"source": 1, "target": 8}, {"source": 2, "target": 9}, {"source": 4, "target": 14}, {"source": 4, "target": 15}, {"source": 4, "target": 16}, {"source": 6, "target": 10}, {"source": 12, "target": 16}, {"source": 16, "target": 18}, {"source": 17, "target": 19}]"#);

    // Track initialization to prevent multiple runs
    let (initialized, set_initialized) = signal(false);

    // Use Effect::new for initialization
    Effect::new(move |_| {
        if !initialized.get() {
            let nodes_str = nodes_data.get_untracked();
            let links_str = links_data.get_untracked();
            
            // Initialize D3 graph when component mounts
            init_d3_graph(nodes_str, links_str);
            
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

            simulation.on("tick", () => {{
                link.attr("d", d => {{
                    const dx = d.target.x - d.source.x;
                    const dy = d.target.y - d.source.y;
                    const dr = Math.sqrt(dx * dx + dy * dy) * 1.5;
                    return `M${{d.source.x}},${{d.source.y}} A${{dr}},${{dr}} 0 0,1 ${{d.target.x}},${{d.target.y}}`;
                }});
                node.attr("transform", d => `translate(${{ d.x }},${{ d.y }})`);
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
