import csv
import json

def csv_to_graph_data(csv_path):
    nodes = []
    links = []

    with open(csv_path, newline='') as csvfile:
        reader = list(csv.DictReader(csvfile))
        for index, row in enumerate(reader):
            nodes.append({
                "id": index,
                "name": row["repository_name"],
                "url": row["repository_url"]
            })

        for index, row in enumerate(reader):
            try:
                children = json.loads(row["child_repository_index"])
                for child_index in children:
                    links.append({
                        "source": index,
                        "target": child_index
                    })
            except Exception as e:
                print(f"Error parsing children at line {index + 1}: {e}")

    return nodes, links

def generate_html(nodes, links, output_html_path):
    import json
    with open(output_html_path, 'w', encoding='utf-8') as f:
        f.write("""
<!DOCTYPE html>
<html>
<head>
  <meta charset="utf-8">
  <title>GitHub Repository Graph</title>
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
  <button id="zoom-out">-</button>
  <button id="zoom-fit">[ ]</button>
</div>
<svg width="100%" height="100%" style="position:absolute;"></svg>
<script>
  const nodes = {nodes_json};
  const links = {links_json};

  const width = window.innerWidth;
  const height = window.innerHeight;

  const svg = d3.select("svg")
    .attr("viewBox", [0, 0, width, height]);

  const zoomLayer = svg.append("g").attr("class", "zoom-layer");

  const link = zoomLayer.append("g")
    .selectAll("line")
    .data(links)
    .join("line")
    .attr("class", "link");

  const simulation = d3.forceSimulation(nodes)
    .force("link", d3.forceLink(links).id(d => d.id).distance(150))
    .force("charge", d3.forceManyBody().strength(-400))
    .force("center", d3.forceCenter(width / 2, height / 2));

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
    link
      .attr("x1", d => d.source.x)
      .attr("y1", d => d.source.y)
      .attr("x2", d => d.target.x)
      .attr("y2", d => d.target.y);

    node
      .attr("transform", d => `translate(${{d.x}},${{d.y}})`);
  }});

  function drag(simulation) {{
    function dragstarted(event, d) {{
      if (!event.active) simulation.alphaTarget(0.3).restart();
      d.fx = d.x;
      d.fy = d.y;
    }}

    function dragged(event, d) {{
      d.fx = event.x;
      d.fy = event.y;
    }}

    function dragended(event, d) {{
      if (!event.active) simulation.alphaTarget(0);
      d.fx = null;
      d.fy = null;
    }}

    return d3.drag()
      .on("start", dragstarted)
      .on("drag", dragged)
      .on("end", dragended);
  }}

  let scale = 1;
  const zoomStep = 0.1;
  const zoomMin = 0.1;
  const zoomMax = 3;
  const baseTranslate = [0, 0];

  function applyZoom() {{
    zoomLayer.attr("transform", `translate(${{baseTranslate[0]}},${{baseTranslate[1]}}) scale(${{scale}})`);
  }}

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

    const boundsWidth = maxX - minX + 150;
    const boundsHeight = maxY - minY + 150;

    const scaleX = width / boundsWidth;
    const scaleY = height / boundsHeight;
    scale = Math.min(scaleX, scaleY, zoomMax);

    const centerX = (minX + maxX) / 2;
    const centerY = (minY + maxY) / 2;
    baseTranslate[0] = width / 2 - scale * centerX;
    baseTranslate[1] = height / 2 - scale * centerY;

    applyZoom();
  }});
</script>
</body>
</html>
""".format(
    nodes_json=json.dumps(nodes),
    links_json=json.dumps(links)
))

if __name__ == "__main__":
    input_csv = "repos.csv"
    output_html = "graph.html"
    nodes, links = csv_to_graph_data(input_csv)
    generate_html(nodes, links, output_html)
    print(f"Graph generated in {output_html}")
