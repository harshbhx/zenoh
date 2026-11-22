#![cfg(feature = "unstable")]
use std::time::Instant;
use zenoh::Wait;
use zenoh_config::WhatAmI;

#[test]
fn join_many_clients() {
    let mut c = zenoh::Config::default();
    c.set_mode(Some(WhatAmI::Router)).unwrap();
    c.listen.endpoints.set(vec!["tcp/127.0.0.1:17999".parse().unwrap()]).unwrap();
    c.scouting.multicast.set_enabled(Some(false)).unwrap();
    c.scouting.gossip.set_enabled(Some(false)).unwrap();
    let router = zenoh::open(c).wait().unwrap();
    let t = Instant::now();
    let handles: Vec<_> = (0..48).map(|_| std::thread::spawn(|| {
        let mut c = zenoh::Config::default();
        c.set_mode(Some(WhatAmI::Client)).unwrap();
        c.connect.endpoints.set(vec!["tcp/127.0.0.1:17999".parse().unwrap()]).unwrap();
        c.scouting.multicast.set_enabled(Some(false)).unwrap();
        zenoh::open(c).wait().unwrap()
    })).collect();
    let sessions: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();
    println!("JOIN_TIME_MS {}", t.elapsed().as_millis());
    drop(sessions);
    router.close().wait().unwrap();
}
