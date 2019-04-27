#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Command {
    Execute,
    CloseWindow,
    SwapTags,
    GotoTag,
    MoveWindowUp,
    MoveWindowDown,
    FocusWindowUp,
    FocusWindowDown,
    FocusWorkspaceNext,
    FocusWorkspacePrevious,
    MoveToTag,
    MoveToLastWorkspace,
    MouseMoveWindow,
    NextLayout,
    PreviousLayout,
}
