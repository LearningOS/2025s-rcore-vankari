//! Types related to task management

use super::TaskContext;

/// The task control block (TCB) of a task.
#[derive(Copy, Clone)]
pub struct TaskControlBlock {
    /// The task status in it's lifecycle
    pub task_status: TaskStatus,
    /// The task context
    pub task_cx: TaskContext,
}

/// The status of a task
#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    /// uninitialized
    UnInit,
    /// ready to run
    Ready,
    /// running
    Running,
    /// exited
    Exited,
}

impl TaskControlBlock {
    /// 该函数将初始化任务的状态为 `UnInit`
    /// 返回一个新的 `TaskControlBlock` 实例。
    pub fn new() -> Self {
        TaskControlBlock {
            task_status: TaskStatus::UnInit,  // 初始化为 UnInit
            task_cx: TaskContext::zero_init(),  // 初始化上下文
        }
    }
}


