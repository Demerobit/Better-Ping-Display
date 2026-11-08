fn inject(mods: &mut Vec<NativeMod>) {
    mods.push(NativeMod {
        id: "better_ping_display".to_owned(),
        title: "Better Ping Display 0.1.0".to_owned(),
        description: "Shows numeric ping in the server list and tab list.".to_owned(),
        keys: Vec::new(),
    });
}
