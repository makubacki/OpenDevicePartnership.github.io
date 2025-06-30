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
    with open(output_html_path, 'w') as f:
        f.write("""
<!DOCTYPE html>
<html>
<head>
  <meta charset="utf-8">
  <title>GitHub Repository Graph</title>
  <script src="https://d3js.org/d3.v7.min.js"></script>
  <style>
    body {{ font-family: sans-serif; }}
    .node rect {{
        stroke: #333;
        fill: skyblue;
        rx: 10px;
        ry: 10px;
        cursor: pointer;
        stroke-width: 1.5px;
    }}
    .node text {{
        pointer-events: none;
        font-size: 13px;
        fill: #000;
        font-weight: bold;
    }}
    .link {{
        fill: none;
        stroke: #888;
        stroke-opacity: 0.6;
        stroke-width: 2px;
    }}
  </style>
</head>
<body>
<svg width="960" height="600"></svg>
<script>
  const nodes = {nodes_json};
  const links = {links_json};

  const width = 960;
  const height = 600;

  const svg = d3.select("svg")
    .attr("viewBox", [0, 0, width, height]);

  const simulation = d3.forceSimulation(nodes)
      .force("link", d3.forceLink(links).id(d => d.id).distance(150))
      .force("charge", d3.forceManyBody().strength(-400))
      .force("center", d3.forceCenter(width / 2, height / 2));

  const link = svg.append("g")
      .attr("stroke", "#999")
      .attr("stroke-opacity", 0.6)
    .selectAll("line")
    .data(links)
    .join("line")
      .attr("stroke-width", 2);

  const node = svg.append("g")
    .selectAll("g")
    .data(nodes)
    .join("g")
    .attr("class", "node")
    .call(drag(simulation));

  node.append("rect")
      .attr("width", 140)
      .attr("height", 40)
      .attr("x", -70)
      .attr("y", -20)
      .on("click", (event, d) => window.open(d.url, '_blank'));

  node.append("text")
      .attr("text-anchor", "middle")
      .attr("dy", "0.35em")
      .text(d => d.name);

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
