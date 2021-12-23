use newam_mqtt::v3::{Publish, PublishBuilder, PublishDe, QoS};
use std::fmt::Write;

const PUBLISH: Publish<30> = PublishBuilder::new()
    .set_qos(QoS::AtMostOnce)
    .set_topic("/test/topic")
    .finalize();

#[test]
fn publish() {
    let mut mypub: Publish<30> = PUBLISH;
    write!(&mut mypub, "Hello, {}!", "world").unwrap();

    let pubde: PublishDe = PublishDe::new(mypub.as_slice()).unwrap();

    assert!(!pubde.dup_flag);
    assert!(!pubde.retain);
    assert_eq!(pubde.qos_level, Some(QoS::AtMostOnce));
    assert_eq!(pubde.topic, Ok("/test/topic"));
    assert_eq!(pubde.payload, b"Hello, world!");
}
