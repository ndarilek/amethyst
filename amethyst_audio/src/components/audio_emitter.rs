use std::{
    collections::HashMap,
    io::Cursor,
};

use rodio::{Decoder, Source as RSource, SpatialSink};
use rodio::source::Buffered;

use amethyst_core::specs::{prelude::Component, storage::BTreeStorage};

use crate::{source::Source, DecoderError};

/// An audio source, add this component to anything that emits sound.
#[derive(Default)]
pub struct AudioEmitter {
    pub(crate) sources: HashMap<String, Buffered<Decoder<Cursor<Source>>>>,
    pub(crate) sinks: HashMap<String, SpatialSink>,
}

impl AudioEmitter {
    /// Creates a new AudioEmitter component initialized to the given positions.
    /// These positions will stay synced with Transform if the Transform component is available
    /// on this entity.
    pub fn new() -> AudioEmitter {
        Default::default()
    }

    /// Plays an audio source from this emitter.
    pub fn play(&mut self, name: String, source: &Source) -> Result<(), DecoderError> {
        let source = Decoder::new(Cursor::new(source.clone())).map_err(|_| DecoderError)?.buffered();
        self.sources.insert(name, source);
        Ok(())
    }
}

impl Component for AudioEmitter {
    type Storage = BTreeStorage<Self>;
}
