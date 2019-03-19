#[derive(Eq, PartialEq, Hash, Debug, Clone)]
pub enum Msg {
    RunCommand,
    Workspaces,
    Subscribe,
    Outputs,
    Tree,
    Marks,
    Barconfig,
    Version,
    BindingModes,
    Config,
    Tick,
    Sync,
}

impl From<u32> for Msg {
    fn from(num: u32) -> Self {
        match num {
            0 => Msg::RunCommand,
            1 => Msg::Workspaces,
            2 => Msg::Subscribe,
            3 => Msg::Outputs,
            4 => Msg::Tree,
            5 => Msg::Marks,
            6 => Msg::Barconfig,
            7 => Msg::Version,
            8 => Msg::BindingModes,
            9 => Msg::Config,
            10 => Msg::Tick,
            11 => Msg::Sync,
            _ => panic!("Unknown message type found"),
        }
    }
}

impl From<Msg> for u32 {
    fn from(msg: Msg) -> Self {
        match msg {
            Msg::RunCommand => 0,
            Msg::Workspaces => 1,
            Msg::Subscribe => 2,
            Msg::Outputs => 3,
            Msg::Tree => 4,
            Msg::Marks => 5,
            Msg::Barconfig => 6,
            Msg::Version => 7,
            Msg::BindingModes => 8,
            Msg::Config => 9,
            Msg::Tick => 10,
            Msg::Sync => 11,
        }
    }
}
