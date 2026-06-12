use chrono::{Duration, NaiveDate};
use iced::Length;
use iced::widget::image::Handle;
use plotters::prelude::IntoFont;
use plotters::{
    prelude::{BitMapBackend, CandleStick, ChartBuilder, GREEN, IntoDrawingArea, RED, WHITE},
    style::Color,
};

#[derive(Clone, Debug)]
pub enum Message {}

pub struct PlottersExample {
    handle: Handle,
}

impl PlottersExample {
    pub fn new() -> Self {
        let (width, height, rgba_buffer) = generate_kline_rgba().unwrap();
        Self {
            handle: Handle::from_rgba(width, height, rgba_buffer),
        }
    }
}

impl crate::Widget for PlottersExample {
    type Message = Message;
    fn update(
        &mut self,
        _message: Self::Message,
        _state: &mut crate::State,
    ) -> iced::Task<Self::Message> {
        iced::Task::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        iced::widget::image(self.handle.clone())
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

// 使用 NaiveDate 替代 deprecated 的 Date
fn generate_kline_rgba() -> Result<(u32, u32, Vec<u8>), Box<dyn std::error::Error>> {
    let data = get_data();
    let (width, height) = (1024, 768);

    // 预分配缓冲区
    let pixel_count = (width * height) as usize;
    let mut rgb_buffer = vec![0u8; pixel_count * 3];

    {
        let root = BitMapBackend::with_buffer(&mut rgb_buffer, (width, height)).into_drawing_area();

        root.fill(&WHITE)?;

        // 使用 NaiveDate 处理日期
        let from_date = parse_time(data[29].0) - Duration::days(1);
        let to_date = parse_time(data[0].0) + Duration::days(1);

        // 配置图表
        let mut chart = ChartBuilder::on(&root)
            .x_label_area_size(40)
            .y_label_area_size(40)
            .caption("MSFT Stock Price", ("sans-serif", 50.0).into_font())
            .build_cartesian_2d(from_date..to_date, 110f32..135f32)?;

        // 绘制网格
        chart.configure_mesh().light_line_style(&WHITE).draw()?;

        // 绘制K线
        let candles: Vec<_> = data
            .iter()
            .map(|x| CandleStick::new(parse_time(x.0), x.1, x.2, x.3, x.4, GREEN.filled(), RED, 15))
            .collect();

        chart.draw_series(candles)?;

        root.present()?;
    }

    // 转换为 RGBA (使用更高效的方式)
    let mut rgba_buffer = Vec::with_capacity(pixel_count * 4);
    rgba_buffer.extend(
        rgb_buffer
            .chunks(3)
            .flat_map(|rgb| [rgb[0], rgb[1], rgb[2], 255].into_iter()),
    );

    Ok((width, height, rgba_buffer))
}

// 使用 NaiveDate 替代 Date<Local>
fn parse_time(t: &str) -> NaiveDate {
    NaiveDate::parse_from_str(t, "%Y-%m-%d").unwrap()
}

// 数据可以定义为常量或静态变量
const STOCK_DATA: [(&str, f32, f32, f32, f32); 30] = [
    ("2019-04-25", 130.06, 131.37, 128.83, 129.15),
    ("2019-04-24", 125.79, 125.85, 124.52, 125.01),
    ("2019-04-23", 124.1, 125.58, 123.83, 125.44),
    ("2019-04-22", 122.62, 124.0000, 122.57, 123.76),
    ("2019-04-18", 122.19, 123.52, 121.3018, 123.37),
    ("2019-04-17", 121.24, 121.85, 120.54, 121.77),
    ("2019-04-16", 121.64, 121.65, 120.1, 120.77),
    ("2019-04-15", 120.94, 121.58, 120.57, 121.05),
    ("2019-04-12", 120.64, 120.98, 120.37, 120.95),
    ("2019-04-11", 120.54, 120.85, 119.92, 120.33),
    ("2019-04-10", 119.76, 120.35, 119.54, 120.19),
    ("2019-04-09", 118.63, 119.54, 118.58, 119.28),
    ("2019-04-08", 119.81, 120.02, 118.64, 119.93),
    ("2019-04-05", 119.39, 120.23, 119.37, 119.89),
    ("2019-04-04", 120.1, 120.23, 118.38, 119.36),
    ("2019-04-03", 119.86, 120.43, 119.15, 119.97),
    ("2019-04-02", 119.06, 119.48, 118.52, 119.19),
    ("2019-04-01", 118.95, 119.1085, 118.1, 119.02),
    ("2019-03-29", 118.07, 118.32, 116.96, 117.94),
    ("2019-03-28", 117.44, 117.58, 116.13, 116.93),
    ("2019-03-27", 117.875, 118.21, 115.5215, 116.77),
    ("2019-03-26", 118.62, 118.705, 116.85, 117.91),
    ("2019-03-25", 116.56, 118.01, 116.3224, 117.66),
    ("2019-03-22", 119.5, 119.59, 117.04, 117.05),
    ("2019-03-21", 117.135, 120.82, 117.09, 120.22),
    ("2019-03-20", 117.39, 118.75, 116.71, 117.52),
    ("2019-03-19", 118.09, 118.44, 116.99, 117.65),
    ("2019-03-18", 116.17, 117.61, 116.05, 117.57),
    ("2019-03-15", 115.34, 117.25, 114.59, 115.91),
    ("2019-03-14", 114.54, 115.2, 114.33, 214.59),
];

fn get_data() -> Vec<(&'static str, f32, f32, f32, f32)> {
    STOCK_DATA.to_vec()
}
