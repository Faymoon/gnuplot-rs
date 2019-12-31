pub trait PlotCanvas
{
    fn on_graph_add(&mut self);
    fn on_graph_added(&mut self);

    fn add_pre_plot_command(&mut self, command: &str);
    fn add_plot_arg(&mut self, arg: &str);
    fn add_plot_data(&mut self, data: &str);
}