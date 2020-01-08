use std::ops::Bound;

use crate::plot_canvas::PlotCanvas;
use crate::enums::XAxes2d;
use crate::enums::YAxes2d;

pub enum GraphStyles // to understand how value should be used check that http://www.gnuplot.info/docs_5.2/Gnuplot_5.2.pdf#part.2
{
	// Basic graph styles
	Lines2d(Vec<[f64; 2]>),
	Lines2dExpr(&'static str),
	Lines3d(Vec<[f64; 3]>),
	Dots2d(Vec<[f64; 2]>),
	Dots2dExpr(&'static str),
	Dots3d(Vec<[f64; 3]>),
	Points2d(Vec<[f64; 2]>),
	Points2dExpr(&'static str),
	Points3d(Vec<[f64; 3]>),
	LinesPoints2d(Vec<[f64; 2]>),
	LinesPoints2dExpr(&'static str),
	LinesPoint3d(Vec<[f64; 3]>),
	Impulses2d(Vec<[f64; 2]>),
	Impulses2dExpr(&'static str),
	Impulses3d(Vec<[f64; 3]>),
	Vectors2d(Vec<[f64; 4]>),
	Vectors3d(Vec<[f64; 6]>),
	Labels2d(Vec<([f64; 2], &'static str)>),
	Labels3d(Vec<([f64; 3], &'static str)>),
	Steps(Vec<[f64; 2]>),
	StepsExpr(&'static str),
	FSteps(Vec<[f64; 2]>),
	FStepsExpr(&'static str),
	Histeps(Vec<[f64; 2]>),
	HistepsExpr(&'static str),
	YErrorBars(Vec<[f64; 4]>),
	XErrorBars(Vec<[f64; 4]>),
	XYErrorBars(Vec<[f64; 6]>),
	YErrorLines(Vec<[f64; 4]>),
	XErrorLines(Vec<[f64; 4]>),
	XYErrorLines(Vec<[f64; 6]>),
	FinanceBars(Vec<[f64; 5]>),
	//Surface,
	ParallelAxes(Vec<f64>, usize),

	// Graph styles with fill style available
	Boxes(Vec<[f64; 3]>),
	BoxPlot(Vec<[f64; 4]>),
	Ellipses(Vec<[f64; 5]>),
	HistogramsClustered(Vec<f64>, f64),             //    /\
	HistogramsRowStacked(Vec<f64>),                 //   /  \
	HistogramsColumnStacked(Vec<f64>),              //  /warn\ : different histogram types can not be displayed together
	HistogramsErrorBars(Vec<[f64; 3]>, f64),        // /______\
	RGBAlpha,
	BoxErrorBars(Vec<[f64; 5]>),
	Candlesticks(Vec<[f64; 5]>),
	Filledcurves,
	Image,
	RGBImage,
	BoxXYError(Vec<[f64; 6]>),
	Circles(Vec<[f64; 5]>),
	FillSteps(Vec<[f64; 2]>),
	PM3d,
	ZErrorFill
}

pub struct Graph
{
	data: GraphStyles,
	title: String,
	range_start: Bound<f64>,
	range_end: Bound<f64>,
	axes: (XAxes2d, YAxes2d),
}

impl Graph
{
	pub fn new(data: GraphStyles) -> Self
	{
		Graph
		{
			data,
			title: String::new(),
			range_start: Bound::Unbounded,
			range_end: Bound::Unbounded,
			axes: (XAxes2d::X1, YAxes2d::Y1),
		}
	}

	pub fn axes(mut self, x_axe: XAxes2d, y_axe: YAxes2d) -> Self
	{
		self.axes =(x_axe, y_axe);
		self
	}

	pub fn title(mut self, title: &str) -> Self
	{
		self.set_title(title);
		self
	}

	//pub fn range<T: RangeBounds<f64>>(mut self, range: T) -> Self
	pub fn range(mut self, start: Bound<f64>, end: Bound<f64>) -> Self
	{
		self.range_start = start;
		self.range_end = end;
		/*
		self.range_start =
			match range.start_bound()
			{
				Bound::Unbounded => Bound::Unbounded,
				Bound::Included(&f) => Bound::Included(f),
				Bound::Excluded(&f) => Bound::Excluded(f),
			};
		self.range_end =
			match range.end_bound()
			{
				Bound::Unbounded => Bound::Unbounded,
				Bound::Included(&f) => Bound::Included(f),
				Bound::Excluded(&f) => Bound::Excluded(f),
			};*/
		self
	}

	pub fn set_axes(mut self, x_axe: XAxes2d, y_axe: YAxes2d)
	{
		self.axes = (x_axe, y_axe);
	}

	pub fn set_title(&mut self, title: &str)
	{
		self.title = String::from(title).replace("\"", "\\\"").replace("'", "\\'");
	}

	pub fn set_range(&mut self, start: Bound<f64>, end: Bound<f64>)
	{
		self.range_start = start;
		self.range_end = end;
	}

	pub fn get_axes(&self) -> &(XAxes2d, YAxes2d)
	{
		&self.axes
	}

	pub fn get_title(&self) -> &str
	{
		&self.title
	}

	pub fn get_range(&self) -> (Bound<f64>, Bound<f64>)
	{
		(self.range_start, self.range_end)
	}

	pub fn draw_to<T: PlotCanvas>(&self, canvas: &mut T)
	{
		canvas.on_graph_add();

		let mut plot_arg_style = String::new();
		let mut plot_arg_data = String::new();

		match &self.data
		{
			GraphStyles::Lines2d(data) =>
				{
					plot_arg_data.push_str("'-'");
					plot_arg_style.push_str("lines");
					canvas.add_plot_data(&parse_plot_data(data.iter().map(|x| x as &[f64]).collect()));
				},
			GraphStyles::Lines2dExpr(string) => 
				{
					plot_arg_data.push_str(&string.replacen(char::is_whitespace, "", string.len()));
					plot_arg_style.push_str("lines");
				},
			GraphStyles::Dots2d(data) =>
				{
					plot_arg_data.push_str("'-'");
					plot_arg_style.push_str("dots");
					canvas.add_plot_data(&parse_plot_data(data.iter().map(|x| x as &[f64]).collect()));
				},
			GraphStyles::Dots2dExpr(string) => 
				{
					plot_arg_data.push_str(&string.replacen(char::is_whitespace, "", string.len()));
					plot_arg_style.push_str("dots");
				},
			GraphStyles::Points2d(data) =>
				{
					plot_arg_data.push_str("'-'");
					plot_arg_style.push_str("points");
					canvas.add_plot_data(&parse_plot_data(data.iter().map(|x| x as &[f64]).collect()));
				},
			GraphStyles::Points2dExpr(string) => 
				{
					plot_arg_data.push_str(&string.replacen(char::is_whitespace, "", string.len()));
					plot_arg_style.push_str("points");
				},
			GraphStyles::LinesPoints2d(data) =>
				{
					plot_arg_data.push_str("'-'");
					plot_arg_style.push_str("linespoints");
					canvas.add_plot_data(&parse_plot_data(data.iter().map(|x| x as &[f64]).collect()));
				},
			GraphStyles::LinesPoints2dExpr(string) => 
				{
					plot_arg_data.push_str(&string.replacen(char::is_whitespace, "", string.len()));
					plot_arg_style.push_str("linespoints");
				},
			GraphStyles::Impulses2d(data) =>
				{
					plot_arg_data.push_str("'-'");
					plot_arg_style.push_str("impulses");
					canvas.add_plot_data(&parse_plot_data(data.iter().map(|x| x as &[f64]).collect()));
				},
			GraphStyles::Impulses2dExpr(string) => 
				{
					plot_arg_data.push_str(&string.replacen(char::is_whitespace, "", string.len()));
					plot_arg_style.push_str("impulses");
				},
			GraphStyles::Vectors2d(data) =>
				{
					plot_arg_data.push_str("'-'");
					plot_arg_style.push_str("vectors");
					canvas.add_plot_data(&parse_plot_data(data.iter().map(|x| x as &[f64]).collect()));
				},
			GraphStyles::Labels2d(data) =>
				{
					plot_arg_data.push_str("'-'");
					plot_arg_style.push_str("labels");

					let mut plot_data = String::new();
					for d in data.iter()
					{
						for val in d.0.iter()
						{
							plot_data.push_str(&val.to_string());
							plot_data.push(' ');
						}
						plot_data.push_str(d.1);
						plot_data.push('\n');
					}
					canvas.add_plot_data(&plot_data);
				},
			GraphStyles::Steps(data) =>
				{
					plot_arg_data.push_str("'-'");
					plot_arg_style.push_str("steps");
					canvas.add_plot_data(&parse_plot_data(data.iter().map(|x| x as &[f64]).collect()));
				},
			GraphStyles::StepsExpr(string) => 
				{
					plot_arg_data.push_str(&string.replacen(char::is_whitespace, "", string.len()));
					plot_arg_style.push_str("steps");
				},
			GraphStyles::FSteps(data) =>
				{
					plot_arg_data.push_str("'-'");
					plot_arg_style.push_str("fsteps");
					canvas.add_plot_data(&parse_plot_data(data.iter().map(|x| x as &[f64]).collect()));
				},
			GraphStyles::FStepsExpr(string) => 
				{
					plot_arg_data.push_str(&string.replacen(char::is_whitespace, "", string.len()));
					plot_arg_style.push_str("fsteps");
				},
			GraphStyles::YErrorBars(data) =>
				{
					plot_arg_data.push_str("'-'");
					plot_arg_style.push_str("yerrorbars");
					canvas.add_plot_data(&parse_plot_data(data.iter().map(|x| x as &[f64]).collect()));
				},
			GraphStyles::XErrorBars(data) =>
				{
					plot_arg_data.push_str("'-'");
					plot_arg_style.push_str("xerrorbars");
					canvas.add_plot_data(&parse_plot_data(data.iter().map(|x| x as &[f64]).collect()));
				},
			GraphStyles::XYErrorBars(data) =>
				{
					plot_arg_data.push_str("'-'");
					plot_arg_style.push_str("xyerrorbars");
					canvas.add_plot_data(&parse_plot_data(data.iter().map(|x| x as &[f64]).collect()));
				},
			GraphStyles::YErrorLines(data) =>
				{
					plot_arg_data.push_str("'-'");
					plot_arg_style.push_str("yerrorlines");
					canvas.add_plot_data(&parse_plot_data(data.iter().map(|x| x as &[f64]).collect()));
				},
			GraphStyles::XErrorLines(data) =>
				{
					plot_arg_data.push_str("'-'");
					plot_arg_style.push_str("xerrorlines");
					canvas.add_plot_data(&parse_plot_data(data.iter().map(|x| x as &[f64]).collect()));
				},
			GraphStyles::XYErrorLines(data) =>
				{
					plot_arg_data.push_str("'-'");
					plot_arg_style.push_str("xyerrorlines");
					canvas.add_plot_data(&parse_plot_data(data.iter().map(|x| x as &[f64]).collect()));
				},
			GraphStyles::FinanceBars(data) =>
				{
					plot_arg_data.push_str("'-'");
					plot_arg_style.push_str("financebars");
					canvas.add_plot_data(&parse_plot_data(data.iter().map(|x| x as &[f64]).collect()));
				},
			GraphStyles::ParallelAxes(data, width) =>
				{
					plot_arg_data.push_str("'-'");
					plot_arg_style.push_str("parallelaxes");
					
					let mut plot_data = String::new();

					let mut i: usize = 1;
					for val in data.iter()
					{
						plot_data.push_str(&val.to_string());
						if i % width == 0
						{
							plot_data.push('\n');
						}
						else
						{
							plot_data.push(' ');
						}
						i += 1;
					}
					canvas.add_plot_data(&plot_data);
				},
			GraphStyles::Boxes(data) =>
				{
					plot_arg_data.push_str("'-'");
					plot_arg_style.push_str("boxes");
					canvas.add_plot_data(&parse_plot_data(data.iter().map(|x| x as &[f64]).collect()));
				},
			GraphStyles::BoxPlot(data) =>
				{
					plot_arg_data.push_str("'-'");
					plot_arg_style.push_str("boxplot");
					canvas.add_plot_data(&parse_plot_data(data.iter().map(|x| x as &[f64]).collect()));
				},
			GraphStyles::BoxXYError(data) =>
				{
					plot_arg_data.push_str("'-'");
					plot_arg_style.push_str("boxxyerror");
					canvas.add_plot_data(&parse_plot_data(data.iter().map(|x| x as &[f64]).collect()));
				},
			GraphStyles::Candlesticks(data) =>
				{
					plot_arg_data.push_str("'-'");
					plot_arg_style.push_str("candlesticks");
					canvas.add_plot_data(&parse_plot_data(data.iter().map(|x| x as &[f64]).collect()));
				},
			GraphStyles::Circles(data) =>
				{
					plot_arg_data.push_str("'-'");
					plot_arg_style.push_str("circles");
					canvas.add_plot_data(&parse_plot_data(data.iter().map(|x| x as &[f64]).collect()));
				},
			GraphStyles::Ellipses(data) =>
				{
					plot_arg_data.push_str("'-'");
					plot_arg_style.push_str("ellipses");
					canvas.add_plot_data(&parse_plot_data(data.iter().map(|x| x as &[f64]).collect()));
				},
			GraphStyles::HistogramsClustered(data, gap) =>
				{
					canvas.add_pre_plot_command(&format!("set style histogram clustered gap {}", gap));
					
					plot_arg_data.push_str("'-'");
					plot_arg_style.push_str("histograms");

					let mut plot_data = String::new();
					for val in data.iter()
					{
						plot_data.push_str(&val.to_string());
						plot_data.push('\n');
					}
					canvas.add_plot_data(&plot_data);
				},
			GraphStyles::HistogramsErrorBars(data, gap) =>
				{
					canvas.add_pre_plot_command(&format!("set style histogram errorbars gap {}", gap));
					
					plot_arg_data.push_str("'-'");
					plot_arg_style.push_str("histograms");
					
					canvas.add_plot_data(&parse_plot_data(data.iter().map(|x| x as &[f64]).collect()));
				},
			GraphStyles::HistogramsColumnStacked(data) =>
				{
					canvas.add_pre_plot_command("set style histogram columnstacked");
					
					plot_arg_data.push_str("'-'");
					plot_arg_style.push_str("histograms");
					
					let mut plot_data = String::new();
					for val in data.iter()
					{
						plot_data.push_str(&val.to_string());
						plot_data.push('\n');
					}
					canvas.add_plot_data(&plot_data);
				},
			GraphStyles::HistogramsRowStacked(data) =>
				{
					canvas.add_pre_plot_command("set style histogram rowstacked");
					
					plot_arg_data.push_str("'-'");
					plot_arg_style.push_str("histograms");
					
					let mut plot_data = String::new();
					for val in data.iter()
					{
						plot_data.push_str(&val.to_string());
						plot_data.push('\n');
					}
					canvas.add_plot_data(&plot_data);
				},
			GraphStyles::BoxErrorBars(data) =>
				{		
					plot_arg_data.push_str("'-'");
					plot_arg_style.push_str("boxerrorbars");
					canvas.add_plot_data(&parse_plot_data(data.iter().map(|x| x as &[f64]).collect()));
				},
			_ => unimplemented!(),
		}

		let mut plot_arg = format!("{} axes {}{} with {}", plot_arg_data, self.axes.0.to_string(), self.axes.1.to_string(), plot_arg_style);

		if !self.title.is_empty()
		{
			plot_arg.push_str(&format!(" title '{}'", self.title));
		}

		canvas.add_plot_arg(&plot_arg);

		canvas.on_graph_added();
	}
}

fn parse_plot_data(data: Vec<&[f64]>) -> String
{
	let mut plot_data = String::new();
	for d in data.iter()
	{
		for val in d.iter()
		{
			plot_data.push_str(&val.to_string());
			plot_data.push(' ');
		}
		plot_data.push('\n');
	}
	plot_data
}
