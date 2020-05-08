#[allow(dead_code)]
pub enum Event {
    ServerOutput(Vec<u8>),
    Prompt,
    ServerSend(Vec<u8>),
    ServerInput(String, bool),
    MudOutput(String),
    Output(String),
    Error(String),
    Info(String),
    UserInputBuffer(String),
    Connect(String, u32),
    Connected,
    ProtoEnabled(u8),
    GMCPReceive(String),
    LoadScript(String),
    ScrollUp,
    ScrollDown,
    ScrollBottom,
    Disconnect,
    Redraw,
    Quit,
}
