//! This example showcases a way to make and display horizontal timelines of widgets.
// In this example, the Timeline structure contains the data to display and is part the state of
// the application.
// It should be noted that the TimelineTrait includes a `show` function to generate
// the appropriate GUI elements, it is not a Widget itself.

use iced::{
    slider, Align, Application, Color, Column, Container, Element, Length, Row, Settings, Slider,
    Space, Text,
};
use iced_wgpu::Backend;

use rand::Rng;

use iced_timeline::*;

type InstantType = u32;
type ValueType = u32;
// Rust will make you implement the necessary traits ub irder to use the TimelineTrait.
// The necessary traits are already implemented by u32 in this case.

struct TimelineExample<Instant, Value> {
    pub time_events: Vec<TimeEvent<Instant, Value>>,
    pub window: TimelineWindow<Instant>,
}

// Create your structure to hold your data points, here I chose to use my TimelineExample struc,
// but any other will do...
type MyTimeline = TimelineExample<InstantType, ValueType>;
//... so long as you implement the TimelineTrait on it :
impl<'a> TimelineTrait<'a, InstantType, ValueType, Row<'a, Message>, Message, Backend>
    for MyTimeline
{
    fn time_events(&self) -> &Vec<TimeEvent<InstantType, ValueType>> {
        &self.time_events
    }

    fn window(&self) -> &TimelineWindow<InstantType> {
        &self.window
    }

    fn init_widget(&self) -> Row<'a, Message> {
        Row::new()
            .padding(20)
            // .height(Length::Fill)
            .width(Length::Fill)
            .align_items(Align::Start)
    }

    fn widget_add(
        widget: Row<'a, Message>,
        time_event: &TimeEvent<InstantType, ValueType>,
        offset: InstantType,
    ) -> Row<'a, Message> {
        // println!("SpaceLength: {}", ((time_event.instant - offset) as u16));
        // println!("TextLength: {}", ((time_event.duration) as u16));
        widget
            .push(Space::with_width(Length::FillPortion(
                (time_event.instant - offset) as u16,
            ))) //todo!():maje sure this u16 conversion is correct
            .push(
                Container::new(Text::new(time_event.value.to_string()).width(Length::Fill))
                    .width(Length::FillPortion((time_event.duration) as u16)),
            )
    }

    fn widget_after(widget: Row<'a, Message>, other_args: InstantType) -> Row<'a, Message> {
        widget.push(Space::with_width(Length::FillPortion(other_args as u16)))
    }
}

//=================================================================================================

static CURRTIME: InstantType = 1609442177; //unix time for 2020-12-31, chosen arbitrarily

pub fn main() -> iced::Result {
    Model::run(Settings::default())
}

struct State {
    slider: slider::State,
    zoom: u8,
    time: InstantType,
    timeline: MyTimeline,
}

struct Model {
    state: State,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    SliderChanged(u32),
}

impl iced::Sandbox for Model {
    type Message = Message;

    fn title(&self) -> String {
        String::from("Timelines - Iced")
    }

    fn new() -> Self {
        Model {
            state: State {
                timeline: MyTimeline {
                    window: TimelineWindow {
                        width: 1000,
                        start: CURRTIME - 1000 / 2,
                        end: CURRTIME + 1000 / 2,
                    },
                    time_events: (1..10_000)
                        .map(|n| {
                            let timeoffst = rand::thread_rng().gen_range(1..150);
                            let duration = 200 - timeoffst;
                            TimeEvent {
                                instant: CURRTIME + n * 200 + timeoffst,
                                value: n,
                                duration,
                            }
                        })
                        .collect(),
                },
                slider: slider::State::new(),
                zoom: 100,
                time: CURRTIME,
            },
        }
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::SliderChanged(time) => {
                self.state.time = CURRTIME + time as InstantType;
                self.state.timeline.window.start =
                    self.state.time - self.state.timeline.window.width / 2;
                self.state.timeline.window.end =
                    self.state.time + self.state.timeline.window.width / 2;
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let column = Column::new();
        let e = self.state.timeline.earliest_instant();
        let l = self.state.timeline.latest_instant();

        let slider = Slider::new(
            &mut (self.state.slider),
            (e - CURRTIME) as u32..=(l - CURRTIME) as u32,
            (self.state.time - CURRTIME) as u32,
            Message::SliderChanged,
        )
        .step((l - e) / 1000 / self.state.zoom as u32);

        let _timeline = self.state.timeline.view();

        let var_name: Element<_> = _timeline.into();
        column
            .push(var_name.explain(Color::BLACK))
            .push(slider)
            .into()
    }
}
