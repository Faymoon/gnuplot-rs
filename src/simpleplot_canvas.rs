use std::process;
use std::io::Write;

use crate::plot_canvas::PlotCanvas;
use crate::enums::CoordinateSystem;

pub struct SimplePlotCanvas
{
	gnuplot_stdin: process::ChildStdin,
	pre_plot_commands: String,
	plot_args: Vec<String>,
	plot_data: Vec<String>,
}

impl SimplePlotCanvas
{
	pub fn new(persistent: bool) -> SimplePlotCanvas
	{
		SimplePlotCanvas
		{
			gnuplot_stdin: 
				{
					let mut cmd = process::Command::new("gnuplot");
					if persistent
					{
						cmd.arg("-p");
					}
					cmd.stdin(process::Stdio::piped())
			         //.stdout(process::Stdio::piped()) Could be useful to handle gnuplot errors/warnings
		               .spawn()
					   .expect("Failed to launch gnuplot")
					   .stdin.expect("Failed to get gnuplot STDIN")
				},
			pre_plot_commands: String::new(),
			plot_args: vec![],
			plot_data: vec![],
		}
	}

	pub fn set_coordinate_system(&mut self, sys: CoordinateSystem)
	{
		match sys
		{
			CoordinateSystem::Polar => self.add_pre_plot_command("set polar"),
			CoordinateSystem::Cartesian => self.add_pre_plot_command("unset polar"),
		}
	}

	pub fn plot(&mut self)
	{
		writeln!(self.gnuplot_stdin, "{}", self.pre_plot_commands).expect("Can't plot");
		
		let mut iter = self.plot_args.iter();
		write!(self.gnuplot_stdin, "plot {}", iter.next().expect("Expected at least one value")).expect("Can't plot");

		for arg in iter
		{
			write!(self.gnuplot_stdin, ", {}", arg).expect("Can't plot");
		}
		writeln!(self.gnuplot_stdin, "").expect("Can't plot");

		for data in self.plot_data.iter()
		{
			writeln!(self.gnuplot_stdin, "{}", data).expect("Can't plot"); 
			writeln!(self.gnuplot_stdin, "e").expect("Can't plot");
		}
	}
}

impl PlotCanvas for SimplePlotCanvas
{
	fn on_graph_add(&mut self)
	{

	}

	fn add_pre_plot_command(&mut self, command: &str)
	{
		self.pre_plot_commands.push_str(command);
		self.pre_plot_commands.push('\n');
	}

	fn add_plot_arg(&mut self, arg: &str)
	{
		self.plot_args.push(String::from(arg));
	}

	fn add_plot_data(&mut self, data: &str)
	{
		self.plot_data.push(String::from(data));
	}

	fn on_graph_added(&mut self)
	{
		self.plot();
	}
}