use std::{
    fs,
    io::{self, ErrorKind},
    path::Path,
    time::Duration,
};

use chrono::Local;
use rodio::{
    ChannelCount, Decoder, OutputStreamBuilder, SampleRate, Sink, Source, source::SeekError,
};

fn main() {
    println!("Hello, world!");

    let stream_handle =
        OutputStreamBuilder::open_default_stream().expect("Failed to initialize audio output");
    let mut sink = Sink::connect_new(stream_handle.mixer());
    sink.pause();

    let track_path = Path::new("[Stone in Focus].m4a");

    let _result = load_and_play(&mut sink, track_path);

    loop {}
}

pub fn load_and_play(sink: &mut Sink, track_path: &Path) -> io::Result<()> {
    sink.stop();

    let file = fs::File::open(track_path)?;

    let decoder_result = Decoder::try_from(file);
    let decoder = match decoder_result {
        Err(e) => return Err(io::Error::new(ErrorKind::Other, e.to_string())),
        Ok(d) => d,
    };

    let visualizer_source = visualizer(decoder);
    sink.append(visualizer_source);
    sink.play();

    Ok(())
}

/// Internal function that builds a `Visualizer` object.
pub fn visualizer<I>(input: I) -> Visualizer<I>
where
    I: Source,
{
    Visualizer { input, sample_index: 0 }
}

#[derive(Clone, Debug)]
pub struct Visualizer<I> {
    input: I,
    sample_index: u64,
}

impl<I> Visualizer<I> {
    /// Returns a reference to the inner source.
    #[inline]
    pub fn inner(&self) -> &I {
        &self.input
    }

    /// Returns a mutable reference to the inner source.
    #[inline]
    pub fn inner_mut(&mut self) -> &mut I {
        &mut self.input
    }

    /// Returns the inner source.
    #[inline]
    pub fn into_inner(self) -> I {
        self.input
    }
}

impl<I> Iterator for Visualizer<I>
where
    I: Source,
{
    type Item = I::Item;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        // TODO: process sample data
        // for now, just print the time.
        let now = Local::now();
        println!("sample {} at {}", self.sample_index, now.format("%H:%M:%S%.3f"));

        self.sample_index += 1;

        self.input.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.input.size_hint()
    }
}

impl<I> ExactSizeIterator for Visualizer<I> where I: Source + ExactSizeIterator {}

impl<I> Source for Visualizer<I>
where
    I: Source,
{
    #[inline]
    fn current_span_len(&self) -> Option<usize> {
        self.input.current_span_len()
    }

    #[inline]
    fn channels(&self) -> ChannelCount {
        self.input.channels()
    }

    #[inline]
    fn sample_rate(&self) -> SampleRate {
        self.input.sample_rate()
    }

    #[inline]
    fn total_duration(&self) -> Option<Duration> {
        self.input.total_duration()
    }

    #[inline]
    fn try_seek(&mut self, pos: Duration) -> Result<(), SeekError> {
        self.input.try_seek(pos)
    }
}
