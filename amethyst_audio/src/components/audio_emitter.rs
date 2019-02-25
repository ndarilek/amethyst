use std::{
    collections::HashMap,
    io::Cursor,
};

use rodio::{Decoder, Sample, Source as RSource, SpatialSink};
use rodio::source::Buffered;

use amethyst_core::specs::{prelude::Component, storage::BTreeStorage};

use crate::{source::Source, DecoderError};

/// An audio source, add this component to anything that emits sound.
#[derive(Default)]
pub struct AudioEmitter<T: Sample + Sync > {
    pub(crate) sources: HashMap<String, Box<dyn RSource<Item = T>>>,
    pub(crate) sinks: HashMap<String, SpatialSink>,
}

impl<T> AudioEmitter<T> {
    /// Creates a new AudioEmitter component initialized to the given positions.
    /// These positions will stay synced with Transform if the Transform component is available
    /// on this entity.
    pub fn new() -> AudioEmitter<T> {
        Default::default()
    }

    /// Plays an audio source from this emitter.
    pub fn play(&mut self, name: String, source: &Source) -> Result<(), DecoderError> {
        let source = Decoder::new(Cursor::new(source.clone())).map_err(|_| DecoderError)?.buffered();
        self.sources.insert(name, source);
        Ok(())
    }
}

impl<T> Component for AudioEmitter<T> {
    type Storage = BTreeStorage<Self>;
}
