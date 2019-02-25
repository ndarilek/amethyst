use std::{
    iter::Iterator,
};

use rodio::SpatialSink;

#[cfg(feature = "profiler")]
use thread_profiler::profile_scope;

use amethyst_core::{
    specs::prelude::{
        Entities, Join, Read, ReadStorage, Resources, System, SystemData, WriteStorage,
    },
    transform::GlobalTransform,
};

use crate::{
    components::{AudioEmitter, AudioListener, SourceHolder},
    output::Output,
};

/// Syncs 3D transform data with the audio engine to provide 3D audio.
#[derive(Default)]
pub struct AudioSystem(Output);

impl AudioSystem {
    /// Produces a new AudioSystem that uses the given output.
    pub fn new(output: Output) -> AudioSystem {
        AudioSystem(output)
    }
}

impl<'a> System<'a> for AudioSystem {
    type SystemData = (
        Option<Read<'a, Output>>,
        Entities<'a>,
        ReadStorage<'a, GlobalTransform>,
        ReadStorage<'a, AudioListener>,
        WriteStorage<'a, AudioEmitter>,
    );

    fn run(
        &mut self,
        (output, entities, transform, listener, mut audio_emitter): Self::SystemData,
    ) {
        #[cfg(feature = "profiler")]
        profile_scope!("audio_system");
        // Process emitters and listener.
        if let Some((listener, entity)) = (&listener, &*entities).join().next() {
            if let Some(listener_transform) = transform.get(entity) {
                let listener_transform = listener_transform.0;
                let left_ear_position = listener_transform
                    .transform_point(&listener.left_ear)
                    .to_homogeneous()
                    .xyz();
                let right_ear_position = listener_transform
                    .transform_point(&listener.right_ear)
                    .to_homogeneous()
                    .xyz();
                for (transform, audio_emitter) in (&transform, &mut audio_emitter).join() {
                    let x = transform.0[(0, 3)];
                    let y = transform.0[(1, 3)];
                    let z = transform.0[(2, 3)];
                    let emitter_position = [x, y, z];
                    for (name, source) in &audio_emitter.sources {
                        if !audio_emitter.sinks.contains_key(name) {
                            if let Some(output) = &output {
                                let sink = SpatialSink::new(
                                    &output.device,
                                    emitter_position,
                                    left_ear_position.into(),
                                    right_ear_position.into(),
                                );
                                let source = match source {
                                    SourceHolder::Decoder(s) => s,
                                    SourceHolder::Repeat(s) => s,
                                };
                                sink.append(source.clone());
                                audio_emitter.sinks.insert(name.to_string(), sink);
                            }
                        }
                    }
                    for ref mut sink in audio_emitter.sinks.values_mut() {
                        sink.set_emitter_position(emitter_position);
                        sink.set_left_ear_position(left_ear_position.into());
                        sink.set_right_ear_position(right_ear_position.into());
                    }
                }
            }
        }
    }

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);
        res.insert(self.0.clone());
    }
}
