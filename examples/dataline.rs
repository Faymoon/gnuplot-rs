extern crate gnuplot_rs;

use gnuplot_rs::graph::Graph;
use gnuplot_rs::graph::GraphStyles;
use gnuplot_rs::simpleplot_canvas::SimplePlotCanvas;

fn main()
{
    let mut canvas = SimplePlotCanvas::new(true);

    let graph = Graph::new(GraphStyles::Lines2d(vec![[0.0, 0.0], [5.0, 5.0]])).title("Hello gnuplot-rs'");

    graph.draw_to(&mut canvas);
}