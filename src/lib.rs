// The TimelineTrait and Timeline struct are decoupled here so that you can use different
// structures if you wish to. To use it with the Timeline, simply use impl TimelineTrait for
// your choice of structure.
use iced_graphics::{Backend, Renderer};
use iced_native::Widget;

use std::ops::{Add, Sub};

pub struct TimeEvent<I, V> {
    pub instant: I,
    pub value: V,
    pub duration: I,
}
pub struct TimelineWindow<I> {
    pub width: I,
    pub start: I,
    pub end: I,
}

pub trait TimelineTrait<'w, I, V, W, Message, B>
where
    I: Copy + Ord + Add<Output = I> + Sub<Output = I>,
    V: Copy,
    W: 'w + Widget<Message, Renderer<B>>,
    B: Backend,
{
    fn time_events(&self) -> &Vec<TimeEvent<I, V>>;
    fn window(&self) -> &TimelineWindow<I>;

    fn earliest_instant(&self) -> I {
        self.time_events().first().unwrap().instant

        /* //for an unsorted list, either sort the list every time it changes or use this
        self.time_events().iter().fold(
            self.time_events().first().unwrap().instant,
            |earliest, m| std::cmp::min(earliest, m.instant),
        )
        */
    }

    fn latest_instant(&self) -> I {
        self.time_events().last().unwrap().instant

        /* //for an unsorted list, either sort the list every time it changes or use this
         self.time_events()
            .iter()
            .fold(self.time_events().first().unwrap().instant, |latest, m| {
                std::cmp::max(latest, m.instant)
            })
        */
    }

    fn init_widget(&self) -> W;

    fn widget_after(widget: W, _other_args: I) -> W {
        widget
    }
    fn widget_add(widget: W, time_event: &TimeEvent<I, V>, offset: I) -> W;

    fn view(&self) -> W {
        let widget: W = self.init_widget();

        let (window_start, window_end) = (self.window().start, self.window().end);

        let mut time_events = &self.time_events()[..];
        // assuming the list is increasingly sorted, this gives the index of first event for which
        //the instant is equal to the window **start**, or, if such event doesn't exist, the index
        // where it would exist.
        let start = match time_events.binary_search_by(|t_e| t_e.instant.cmp(&window_start)) {
            Err(s) => s,
            Ok(s) => s,
        };
        // if window is right of all data, dont bother with filling the widget
        if start == time_events.len() {
            widget
        } else {
            //forget about all the events before the window starts.
            time_events = &time_events[start..];

            // assuming the list is increasingly sorted, this gives the index of first event for
            // which the instant is equal to the window **end**, or, if such event doesn't exist,
            // the index where it would exist.
            let end = match time_events.binary_search_by(|t_e| {
                let t = t_e.instant + t_e.duration;
                t.cmp(&window_end)
            }) {
                Ok(e) => e + 1,
                Err(e) => e,
            };

            // Now, we can very efficiently work with the only events that should be displayed
            time_events = &time_events[..end];
            let (space_start, mut time_event) = time_events
                .iter()
                .fold((window_start, widget), |(ss, w), t_e| {
                    ({ t_e.instant + t_e.duration }, Self::widget_add(w, t_e, ss))
                });
            time_event = Self::widget_after(time_event, window_end - space_start);
            time_event
        }
    }
}
