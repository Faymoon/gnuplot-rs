extern crate gnuplot_rs;

use gnuplot_rs::graph::Graph;
use gnuplot_rs::graph::GraphStyles;
use gnuplot_rs::simpleplot_canvas::SimplePlotCanvas;
use gnuplot_rs::enums::CoordinateSystem;
use gnuplot_rs::enums::XAxes2d;
use gnuplot_rs::enums::YAxes2d;

fn main()
{
    let mut canvas = SimplePlotCanvas::new(true);

    canvas.set_coordinate_system(CoordinateSystem::Cartesian); // this requires that expressions for graphs will be in function of x

    let graph = Graph::new(GraphStyles::Lines2d(vec![[0.0, 0.0], [5.0, 5.0], [8.0, -1.0]])).axes(XAxes2d::X1, YAxes2d::Y2);
    let sinus = Graph::new(GraphStyles::Lines2dExpr("sin(x)"));
    let test = Graph::new(GraphStyles::Lines2dExpr("cos(x)"));

    test.draw_to(&mut canvas);
    sinus.draw_to(&mut canvas);
    graph.draw_to(&mut canvas);
}