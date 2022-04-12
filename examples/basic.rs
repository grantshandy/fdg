use fdg::{Node, Color};

fn main() {
    let mut nodes: Vec<&Node> = Vec::new();

    let one = Node::new("One", Color::RED);
    nodes.push(&one);

    let two = Node::new("Two", Color::BLUE);
    nodes.push(&two);

    // let links: Vec<[Node; 2]> = Vec::new();

    let mut links: Vec<[&Node; 2]> = Vec::new();
    links.push([&one, &two]);

    pollster::block_on(fdg::run(nodes, links));
}
