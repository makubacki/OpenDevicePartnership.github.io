import csv
import json

def csv_to_graph_data(csv_path):
    nodes = []
    links = []
    classifications = {}

    with open(csv_path, newline='', encoding='utf-8') as csvfile:
        reader = list(csv.DictReader(csvfile))
        for i, row in enumerate(reader):
            classification = row["classification"]
            order = int(row["classification_order"])
            classifications[classification] = order

            nodes.append({
                "id": i,
                "name": row["repository_name"],
                "url": row["repository_url"],
                "classification": classification,
                "order": order,
            })

        for i, row in enumerate(reader):
            children = json.loads(row["child_repository_index"])
            for child_index in children:
                links.append({"source": i, "target": child_index})

    return nodes, links

def generate_html(nodes, links, output_html_path):
    nodes_json = json.dumps(nodes)
    links_json = json.dumps(links)

    html_content = f'''
<!DOCTYPE html>
<html>
<head>
  <meta charset="utf-8">
  <title>Repository Graph</title>
  <script src="https://d3js.org/d3.v7.min.js"></script>
  <style>
    body {{
      font-family: sans-serif;
      margin: 0;
      overflow: hidden;
    }}
    .link {{
      fill: none;
      stroke: #888;
      stroke-opacity: 0.6;
      stroke-width: 2px;
    }}
    .node text {{
      pointer-events: none;
      font-size: 13px;
      fill: #000;
      font-weight: bold;
    }}
    #zoom-controls {{
      position: absolute;
      top: 10px;
      left: 10px;
      background: white;
      border: 1px solid #ccc;
      border-radius: 6px;
      box-shadow: 1px 1px 5px rgba(0,0,0,0.1);
      padding: 4px;
      z-index: 10;
    }}
    #zoom-controls button {{
      font-size: 18px;
      width: 35px;
      height: 30px;
      margin: 2px;
      cursor: pointer;
    }}
  </style>
</head>
<body>
<div id="zoom-controls">
  <button id="zoom-in">+</button>
  <button id="zoom-out">−</button>
  <button id="zoom-fit">⊞</button>
  <button id="open-new">⛶</button>
</div>
<svg width="100%" height="100%" style="position:absolute;"></svg>
<script>
  const nodes = {nodes_json};
  const links = {links_json};

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

  let scale = 1;
  const zoomStep = 0.1;
  const zoomMin = 0.1;
  const zoomMax = 3;
  const baseTranslate = [0, 0];

  function applyZoom() {{
    zoomLayer.attr("transform", `translate(${{ baseTranslate[0] }},${{ baseTranslate[1] }}) scale(${{ scale }})`);
  }}

  svg.call(
    d3.zoom()
      .scaleExtent([zoomMin, zoomMax])
      .on("zoom", (event) => {{
        zoomLayer.attr("transform", event.transform);
        scale = event.transform.k;
        baseTranslate[0] = event.transform.x;
        baseTranslate[1] = event.transform.y;
      }})
  );

  d3.select("#zoom-in").on("click", () => {{
    scale = Math.min(scale + zoomStep, zoomMax);
    applyZoom();
  }});

  d3.select("#zoom-out").on("click", () => {{
    scale = Math.max(scale - zoomStep, zoomMin);
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
    scale = Math.min(scaleX, scaleY, zoomMax);

    const centerX = (minX + maxX) / 2;
    const centerY = (minY + maxY) / 2;
    baseTranslate[0] = width / 2 - scale * centerX;
    baseTranslate[1] = height / 2 - scale * centerY;

    applyZoom();
  }});

  d3.select("#open-new").on("click", () => {{
    window.open("about:blank", "_blank");
  }});
</script>
</body>
</html>
'''

    with open(output_html_path, 'w', encoding='utf-8') as f:
        f.write(html_content)

if __name__ == "__main__":
    input_csv = "repos.csv"
    output_html = "graph.html"
    nodes, links = csv_to_graph_data(input_csv)
    generate_html(nodes, links, output_html)
