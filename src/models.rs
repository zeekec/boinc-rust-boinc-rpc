use super::util;
use serde::{Deserialize, Serialize};
use treexml;

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Component {
    CPU,
    GPU,
    Network,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum RunMode {
    Always,
    Auto,
    Never,
    Restore,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum CpuSched {
    Uninitialized,
    Preempted,
    Scheduled,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum ResultState {
    New,
    FilesDownloading,
    FilesDownloaded,
    ComputeError,
    FilesUploading,
    FilesUploaded,
    Aborted,
    UploadFailed,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Process {
    Uninitialized = 0,
    Executing = 1,
    Suspended = 9,
    AbortPending = 5,
    QuitPending = 8,
    CopyPending = 10,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct VersionInfo {
    pub major: Option<i64>,
    pub minor: Option<i64>,
    pub release: Option<i64>,
}

impl From<&treexml::Element> for VersionInfo {
    fn from(node: &treexml::Element) -> Self {
        let mut e = Self::default();
        for n in &node.children {
            match &*n.name {
                "major" => e.major = util::eval_node_contents(n),
                "minor" => e.minor = util::eval_node_contents(n),
                "release" => e.release = util::eval_node_contents(n),
                _ => {}
            }
        }
        e
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct HostInfo {
    pub tz_shift: Option<i64>,
    pub domain_name: Option<String>,
    pub serialnum: Option<String>,
    pub ip_addr: Option<String>,
    pub host_cpid: Option<String>,

    pub p_ncpus: Option<i64>,
    pub p_vendor: Option<String>,
    pub p_model: Option<String>,
    pub p_features: Option<String>,
    pub p_fpops: Option<f64>,
    pub p_iops: Option<f64>,
    pub p_membw: Option<f64>,
    pub p_calculated: Option<f64>,
    pub p_vm_extensions_disabled: Option<bool>,

    pub m_nbytes: Option<f64>,
    pub m_cache: Option<f64>,
    pub m_swap: Option<f64>,

    pub d_total: Option<f64>,
    pub d_free: Option<f64>,

    pub os_name: Option<String>,
    pub os_version: Option<String>,
    pub product_name: Option<String>,

    pub mac_address: Option<String>,

    pub virtualbox_version: Option<String>,
}

impl From<&treexml::Element> for HostInfo {
    fn from(node: &treexml::Element) -> Self {
        let mut e = Self::default();
        for n in &node.children {
            match &*n.name {
                "p_fpops" => e.p_fpops = util::eval_node_contents(n),
                "p_iops" => e.p_iops = util::eval_node_contents(n),
                "p_membw" => e.p_membw = util::eval_node_contents(n),
                "p_calculated" => e.p_calculated = util::eval_node_contents(n),
                "p_vm_extensions_disabled" => {
                    e.p_vm_extensions_disabled = util::eval_node_contents(n);
                }

                "host_cpid" => e.host_cpid.clone_from(&n.text),
                "product_name" => e.product_name.clone_from(&n.text),
                "mac_address" => e.mac_address.clone_from(&n.text),
                "domain_name" => e.domain_name.clone_from(&n.text),
                "ip_addr" => e.ip_addr.clone_from(&n.text),
                "p_vendor" => e.p_vendor.clone_from(&n.text),
                "p_model" => e.p_model.clone_from(&n.text),
                "os_name" => e.os_name.clone_from(&n.text),
                "os_version" => e.os_version.clone_from(&n.text),
                "virtualbox_version" => e.virtualbox_version.clone_from(&n.text),
                "p_features" => e.p_features.clone_from(&n.text),
                "timezone" => e.tz_shift = util::eval_node_contents(n),
                "p_ncpus" => e.p_ncpus = util::eval_node_contents(n),
                "m_nbytes" => e.m_nbytes = util::eval_node_contents(n),
                "m_cache" => e.m_cache = util::eval_node_contents(n),
                "m_swap" => e.m_swap = util::eval_node_contents(n),
                "d_total" => e.d_total = util::eval_node_contents(n),
                "d_free" => e.d_free = util::eval_node_contents(n),
                _ => {}
            }
        }
        e
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ProjectInfo {
    pub name: Option<String>,
    pub summary: Option<String>,
    pub url: Option<String>,
    pub general_area: Option<String>,
    pub specific_area: Option<String>,
    pub description: Option<String>,
    pub home: Option<String>,
    pub platforms: Option<Vec<String>>,
    pub image: Option<String>,
}

impl From<&treexml::Element> for ProjectInfo {
    fn from(node: &treexml::Element) -> Self {
        let mut e = Self::default();
        for n in &node.children {
            match &*n.name {
                "name" => {
                    e.name = util::trimmed_optional(&util::any_text(n));
                }
                "summary" => {
                    e.summary = util::trimmed_optional(&util::any_text(n));
                }
                "url" => {
                    e.url = util::trimmed_optional(&util::any_text(n));
                }
                "general_area" => {
                    e.general_area = util::trimmed_optional(&util::any_text(n));
                }
                "specific_area" => {
                    e.specific_area = util::trimmed_optional(&util::any_text(n));
                }
                "description" => {
                    e.description = util::trimmed_optional(&util::any_text(n));
                }
                "home" => {
                    e.home = util::trimmed_optional(&util::any_text(n));
                }
                "platfroms" => {
                    let mut platforms = Vec::new();
                    for platform_node in &n.children {
                        if platform_node.name == "platform" {
                            if let Some(v) = &platform_node.text {
                                platforms.push(v.clone());
                            }
                        }
                    }
                    e.platforms = Some(platforms);
                }
                "image" => {
                    e.image = util::trimmed_optional(&util::any_text(n));
                }
                _ => {}
            }
        }

        e
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct AccountManagerInfo {
    pub url: Option<String>,
    pub name: Option<String>,
    pub have_credentials: Option<bool>,
    pub cookie_required: Option<bool>,
    pub cookie_failure_url: Option<String>,
}

impl From<&treexml::Element> for AccountManagerInfo {
    fn from(node: &treexml::Element) -> Self {
        let mut e = Self::default();
        for n in &node.children {
            match &*n.name {
                "acct_mgr_url" => e.url = util::trimmed_optional(&util::any_text(n)),
                "acct_mgr_name" => e.name = util::trimmed_optional(&util::any_text(n)),
                "have_credentials" => {
                    e.have_credentials = Some(true);
                }
                "cookie_required" => {
                    e.cookie_required = Some(true);
                }
                "cookie_failure_url" => {
                    e.cookie_failure_url = util::trimmed_optional(&util::any_text(n));
                }
                _ => {}
            }
        }
        e
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Message {
    pub project_name: Option<String>,
    pub priority: Option<i64>,
    pub msg_number: Option<i64>,
    pub body: Option<String>,
    pub timestamp: Option<i64>,
}

impl From<&treexml::Element> for Message {
    fn from(node: &treexml::Element) -> Self {
        let mut e = Self::default();
        for n in &node.children {
            match &*n.name {
                "body" => {
                    e.body = util::trimmed_optional(&n.cdata);
                }
                "project" => {
                    e.project_name = util::trimmed_optional(&n.text);
                }
                "pri" => {
                    e.priority = util::eval_node_contents(n);
                }
                "seqno" => {
                    e.msg_number = util::eval_node_contents(n);
                }
                "time" => {
                    e.timestamp = util::eval_node_contents(n);
                }
                _ => {}
            }
        }

        e
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct TaskResult {
    pub name: Option<String>,
    pub wu_name: Option<String>,
    pub platform: Option<String>,
    pub version_num: Option<i64>,
    pub plan_class: Option<String>,
    pub project_url: Option<String>,
    pub final_cpu_time: Option<f64>,
    pub final_elapsed_time: Option<f64>,
    pub exit_status: Option<i64>,
    pub state: Option<i64>,
    pub report_deadline: Option<f64>,
    pub received_time: Option<f64>,
    pub estimated_cpu_time_remaining: Option<f64>,
    pub completed_time: Option<f64>,
    pub active_task: Option<ActiveTask>,
}

impl From<&treexml::Element> for TaskResult {
    fn from(node: &treexml::Element) -> Self {
        let mut e = Self::default();
        for n in &node.children {
            match &*n.name {
                "name" => {
                    e.name = util::trimmed_optional(&n.text);
                }
                "wu_name" => {
                    e.wu_name = util::trimmed_optional(&n.text);
                }
                "platform" => {
                    e.platform = util::trimmed_optional(&n.text);
                }
                "version_num" => {
                    e.version_num = util::eval_node_contents(n);
                }
                "plan_class" => {
                    e.plan_class = util::trimmed_optional(&n.text);
                }
                "project_url" => {
                    e.project_url = util::trimmed_optional(&n.text);
                }
                "final_cpu_time" => {
                    e.final_cpu_time = util::eval_node_contents(n);
                }
                "final_elapsed_time" => {
                    e.final_elapsed_time = util::eval_node_contents(n);
                }
                "exit_status" => {
                    e.exit_status = util::eval_node_contents(n);
                }
                "state" => {
                    e.state = util::eval_node_contents(n);
                }
                "report_deadline" => {
                    e.report_deadline = util::eval_node_contents(n);
                }
                "received_time" => {
                    e.received_time = util::eval_node_contents(n);
                }
                "estimated_cpu_time_remaining" => {
                    e.estimated_cpu_time_remaining = util::eval_node_contents(n);
                }
                "completed_time" => {
                    e.completed_time = util::eval_node_contents(n);
                }
                "active_task" => {
                    e.active_task = Some(ActiveTask::from(n));
                }
                _ => {}
            }
        }
        e
    }
}
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ActiveTask {
    pub active_task_state: Option<String>,
    pub app_version_num: Option<String>,
    pub slot: Option<u64>,
    pub pid: Option<u64>,
    pub scheduler_state: Option<String>,
    pub checkpoint_cpu_time: Option<f64>,
    pub fraction_done: Option<f64>,
    pub current_cpu_time: Option<f64>,
    pub elapsed_time: Option<f64>,
    pub swap_size: Option<f64>,
    pub working_set_size: Option<f64>,
    pub working_set_size_smoothed: Option<f64>,
    pub page_fault_rate: Option<f64>,
    pub bytes_sent: Option<f64>,
    pub bytes_received: Option<f64>,
    pub progress_rate: Option<f64>,
}

impl From<&treexml::Element> for ActiveTask {
    fn from(node: &treexml::Element) -> Self {
        let mut e = Self::default();
        for n in &node.children {
            match &*n.name {
                "active_task_state" => {
                    e.active_task_state = util::trimmed_optional(&n.text);
                }
                "app_version_num" => {
                    e.app_version_num = util::trimmed_optional(&n.text);
                }
                "slot" => {
                    e.slot = util::eval_node_contents(n);
                }
                "pid" => {
                    e.pid = util::eval_node_contents(n);
                }
                "scheduler_state" => {
                    e.scheduler_state = util::trimmed_optional(&n.text);
                }
                "checkpoint_cpu_time" => {
                    e.checkpoint_cpu_time = util::eval_node_contents(n);
                }
                "fraction_done" => {
                    e.fraction_done = util::eval_node_contents(n);
                }
                "current_cpu_time" => {
                    e.current_cpu_time = util::eval_node_contents(n);
                }
                "elapsed_time" => {
                    e.elapsed_time = util::eval_node_contents(n);
                }
                "swap_size" => {
                    e.swap_size = util::eval_node_contents(n);
                }
                "working_set_size" => {
                    e.working_set_size = util::eval_node_contents(n);
                }
                "working_set_size_smoothed" => {
                    e.working_set_size_smoothed = util::eval_node_contents(n);
                }
                "page_fault_rate" => {
                    e.page_fault_rate = util::eval_node_contents(n);
                }
                "bytes_sent" => {
                    e.bytes_sent = util::eval_node_contents(n);
                }
                "bytes_received" => {
                    e.bytes_received = util::eval_node_contents(n);
                }
                "progress_rate" => {
                    e.progress_rate = util::eval_node_contents(n);
                }
                _ => {}
            }
        }
        e
    }
}
