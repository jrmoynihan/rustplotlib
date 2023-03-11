use crate::components::axis::{AxisLine, AxisTick};
use crate::scales::ScaleType;
use crate::{Chart, Scale};
use std::string::ToString;
use svg::node::element::Group;
use svg::node::element::Text;
use svg::node::Text as TextNode;
use svg::parser::Error;
use svg::Node;

// Enum for tick label sizing
enum TickLabel {
    Band(usize),
    Linear(f32),
    Ordinal(usize),
}

/// Enum of possible axis positions on the chart.
#[derive(Copy, Clone, PartialEq)]
pub enum AxisPosition {
    Top,
    Right,
    Bottom,
    Left,
}

/// An axis struct that represents an axis along a dimension of the chart.
pub struct Axis {
    ticks: Vec<AxisTick>,
    tick_label_font_size: Option<usize>,
    max_tick_length: TickLabel,
    axis_line: AxisLine,
    position: AxisPosition,
    label: String,
    label_rotation: isize,
    label_format: String,
    label_font_size: String,
    length: isize,
}

impl Axis {
    /// Create a new instance of an axis for a chart based on the provided scale and position.
    fn new<'a, T: ToString>(
        scale: &'a dyn Scale<T>,
        position: AxisPosition,
        chart: &Chart<'a>,
    ) -> Self {
        Self {
            ticks: Self::generate_ticks(scale, position),
            tick_label_font_size: None,
            max_tick_length: Self::calculate_max_tick_length(scale),
            position,
            axis_line: Self::get_axis_line(position, chart),
            label: String::new(),
            label_rotation: 0,
            label_format: String::new(),
            length: Self::get_axis_length(position, chart),
            label_font_size: "14px".to_owned(),
        }
    }

    /// Create a new axis at the top of the chart.
    pub fn new_top_axis<'a, T: ToString>(scale: &'a dyn Scale<T>, chart: &Chart<'a>) -> Self {
        Self::new(scale, AxisPosition::Top, chart)
    }

    /// Create a new axis to the right of the chart.
    pub fn new_right_axis<'a, T: ToString>(scale: &'a dyn Scale<T>, chart: &Chart<'a>) -> Self {
        Self::new(scale, AxisPosition::Right, chart)
    }

    /// Create a new axis at the bottom of the chart.
    pub fn new_bottom_axis<'a, T: ToString>(scale: &'a dyn Scale<T>, chart: &Chart<'a>) -> Self {
        Self::new(scale, AxisPosition::Bottom, chart)
    }

    /// Create a new axis to the left of the chart.
    pub fn new_left_axis<'a, T: ToString>(scale: &'a dyn Scale<T>, chart: &Chart<'a>) -> Self {
        Self::new(scale, AxisPosition::Left, chart)
    }

    /// Set axis label.
    pub fn set_axis_label(&mut self, label: String) {
        self.label = label;
    }

    /// Set font size for axis label.
    pub fn set_axis_label_font_size(&mut self, size: usize) {
        self.label_font_size = format!("{}px", size);
    }

    /// Set tick label rotation.
    pub fn set_tick_label_rotation(&mut self, rotation: isize) {
        self.label_rotation = rotation;
        self.ticks
            .iter_mut()
            .for_each(|tick| tick.set_label_rotation(rotation));
    }

    /// Set tick label font size.
    pub fn set_tick_label_font_size(&mut self, size: usize) {
        self.tick_label_font_size = Some(size);
        self.ticks
            .iter_mut()
            .for_each(|tick| tick.set_label_font_size(size));
    }

    /// Set the label format.
    pub fn set_tick_label_format(&mut self, format: &str) {
        self.label_format = String::from(format);
        let label_format = self.label_format.as_str();
        self.ticks
            .iter_mut()
            .for_each(|tick| tick.set_label_format(label_format));
    }

    /// Return whether the axis has a label or not.
    pub fn has_label(&self) -> bool {
        !self.label.is_empty()
    }

    /// Compute the length of the axis.
    fn get_axis_length(position: AxisPosition, chart: &Chart<'_>) -> isize {
        if position == AxisPosition::Top || position == AxisPosition::Bottom {
            chart.get_view_width()
        } else {
            chart.get_view_height()
        }
    }

    /// Calculate analogue for the length of the tick labels.
    fn calculate_max_tick_length<T: ToString>(scale: &dyn Scale<T>) -> TickLabel {
        match scale.get_type() {
            ScaleType::Band => {
                match scale
                    .get_domain()
                    .into_iter()
                    .map(|s| s.to_string().len())
                    .max()
                {
                    Some(size) => TickLabel::Band(size),
                    None => TickLabel::Band(0),
                }
            }
            ScaleType::Linear => TickLabel::Linear(scale.domain_max()),
            ScaleType::Logarithmic => TickLabel::Linear(scale.domain_max()),

            ScaleType::Ordinal => {
                todo!();
                // When ordinal scale type is implemented
                #[allow(unreachable_code)]
                TickLabel::Ordinal(0)
            }
        }
    }

    /// Calculate the y position for the label
    fn calculate_y_for_label(&self) -> i32 {
        match self.max_tick_length {
            TickLabel::Band(characters) => {
                let calculated = match self.tick_label_font_size {
                    Some(font_size) => Axis::characters_to_px(characters, font_size),
                    None => Axis::characters_to_px(characters, 12),
                };

                match self.position {
                    AxisPosition::Top => 42,
                    AxisPosition::Bottom => 42,
                    AxisPosition::Left => calculated,
                    AxisPosition::Right => calculated,
                }
            }
            TickLabel::Linear(upper_bound) => {
                let characters = format_num::format_num!(&self.label_format, upper_bound).len() + 2;

                let calculated = match self.tick_label_font_size {
                    Some(font_size) => Axis::characters_to_px(characters, font_size),
                    None => Axis::characters_to_px(characters, 12),
                };

                match self.position {
                    AxisPosition::Top => 42,
                    AxisPosition::Bottom => 42,
                    AxisPosition::Left => calculated,
                    AxisPosition::Right => calculated,
                }
            }
            TickLabel::Ordinal(_size) => 42,
        }
    }

    fn characters_to_px(characters: usize, font_size: usize) -> i32 {
        // tick space + characters * fontsize * proportion
        (12_f32 + (characters as f32 * font_size as f32 * 0.7)) as i32
    }

    /// Generate svg for the axis.
    pub fn to_svg(&self) -> Result<Group, Error> {
        let axis_class = match self.position {
            AxisPosition::Top => "x-axis",
            AxisPosition::Bottom => "x-axis",
            AxisPosition::Left => "y-axis",
            AxisPosition::Right => "y-axis",
        };

        let mut group = Group::new()
            .set("class", axis_class)
            .add(self.axis_line.to_svg().unwrap());

        for tick in self.ticks.iter() {
            group.append(tick.to_svg().unwrap());
        }

        if !self.label.is_empty() {
            let (x, y, rotate) = match self.position {
                AxisPosition::Top => (
                    (self.length / 2) as i32,
                    -(self.calculate_y_for_label() - 10),
                    0,
                ),
                AxisPosition::Bottom => ((self.length / 2) as i32, self.calculate_y_for_label(), 0),
                AxisPosition::Left => (
                    -(self.length as i32 / 2),
                    -self.calculate_y_for_label(),
                    -90,
                ),
                AxisPosition::Right => {
                    ((self.length as i32 / 2), -self.calculate_y_for_label(), 90)
                }
            };
            let axis_label = Text::new()
                .set("x", x)
                .set("y", y)
                .set("text-anchor", "middle")
                .set("font-size", self.label_font_size.clone())
                .set("font-family", "sans-serif")
                .set("fill", "#777")
                .set("transform", format!("rotate({})", rotate))
                .add(TextNode::new(&self.label));
            group.append(axis_label);
        }

        Ok(group)
    }

    /// Generate ticks for the axis based on the scale and position.
    fn generate_ticks<T: ToString>(scale: &dyn Scale<T>, position: AxisPosition) -> Vec<AxisTick> {
        let mut ticks = Vec::new();
        let label_offset = {
            if position == AxisPosition::Top || position == AxisPosition::Bottom {
                16
            } else {
                12
            }
        };

        for tick in scale.get_ticks() {
            let tick_offset = match position {
                AxisPosition::Bottom if scale.get_type() == ScaleType::Band => {
                    scale.scale(&tick) + scale.bandwidth().unwrap() / 2_f32
                }
                AxisPosition::Bottom => scale.scale(&tick),
                AxisPosition::Left if scale.get_type() == ScaleType::Band => {
                    scale.scale(&tick) + scale.bandwidth().unwrap() / 2_f32
                }
                AxisPosition::Left => scale.scale(&tick),
                AxisPosition::Top if scale.get_type() == ScaleType::Band => {
                    scale.scale(&tick) + scale.bandwidth().unwrap() / 2_f32
                }
                AxisPosition::Top => scale.scale(&tick),
                AxisPosition::Right if scale.get_type() == ScaleType::Band => {
                    scale.scale(&tick) + scale.bandwidth().unwrap() / 2_f32
                }
                AxisPosition::Right => scale.scale(&tick),
            };
            let axis_tick = AxisTick::new(
                tick_offset,
                label_offset,
                0,
                tick.to_string(),
                None,
                position,
            );
            ticks.push(axis_tick);
        }

        ticks
    }

    /// Generate the line that represents the axis.
    fn get_axis_line(position: AxisPosition, chart: &Chart<'_>) -> AxisLine {
        match position {
            AxisPosition::Top => AxisLine::new(0_f32, 0_f32, chart.get_view_width() as f32, 0_f32),
            AxisPosition::Right => {
                AxisLine::new(0_f32, 0_f32, 0_f32, chart.get_view_height() as f32)
            }
            AxisPosition::Bottom => {
                AxisLine::new(0_f32, 0_f32, chart.get_view_width() as f32, 0_f32)
            }
            AxisPosition::Left => {
                AxisLine::new(0_f32, 0_f32, 0_f32, chart.get_view_height() as f32)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_px() {
        let string = format_num::format_num!(".3%", 0.7);
        let characters = string.len();
        let font_size = 14;
        let px = Axis::characters_to_px(characters, font_size);

        println!(
            "With the string {} we have characters {} and font size {} we get px {}",
            string, characters, font_size, px
        );

        assert_eq!(px, 80);
    }
}
