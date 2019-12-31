pub enum CoordinateSystem
{
	Polar,
	Cartesian,
}

pub enum XAxes2d
{
	X1,
	X2,
}

pub enum YAxes2d
{
	Y1,
	Y2,
}

impl XAxes2d
{
    pub(crate) fn to_string(&self) -> String
    {
        match self
        {
            XAxes2d::X1 => String::from("x1"),
            XAxes2d::X2 => String::from("x2"),
        }
    }
}

impl YAxes2d
{
    pub(crate) fn to_string(&self) -> String
    {
        match self
        {
            YAxes2d::Y1 => String::from("y1"),
            YAxes2d::Y2 => String::from("y2"),
        }
    }
}