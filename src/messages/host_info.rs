use quick_xml;
use serde::{Deserialize, Serialize};
use std::{
    fmt::{Display, Formatter, Result},
    i32,
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum DockerType {
    #[serde(rename = "docker")]
    Docker,
    #[serde(rename = "podman")]
    Podman,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "get_host_info")]
pub struct GetHostInfo {}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "host_info")]
pub struct HostInfo {
    pub tz_shift: Option<i32>,
    pub domain_name: Option<String>,
    pub serialnum: Option<String>,
    pub ip_addr: Option<String>,
    pub host_cpid: Option<String>,

    pub p_ncpus: Option<i32>,
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

    // pub wsl_distro: Option<WslDistros>, // TODO: Implement WslDistros, Windoes only
    pub docker_version: Option<String>,
    pub docker_type: Option<DockerType>,
    pub docker_compose_version: Option<String>,
    pub docker_compose_type: Option<DockerType>,

    pub product_name: Option<String>,
    pub mac_address: Option<String>,

    pub virtualbox_version: Option<String>,

    // pub coprocs: Option<CoProcs>, // TODO: Implement CoProcs (i.e. GPUs)
    pub num_opencl_cpu_platforms: Option<i32>,
    // pub opencl_cpu_prop: Option<Vec<OpenClCpuProp>>, // TODO: Implement OpenClCpuProp
}

impl Display for HostInfo {
    fn fmt(&self, f: &mut Formatter) -> Result {
        use quick_xml::se::Serializer;
        use serde::Serialize;

        let mut buffer = String::new();
        let mut ser = Serializer::new(&mut buffer);
        ser.indent(' ', 4);

        self.serialize(ser).unwrap();

        write!(f, "{buffer}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quick_xml;
    use testing_logger;

    #[test]
    fn test_format() {
        testing_logger::setup();

        let host_info = HostInfo {
            ..Default::default()
        };
        let xml = format!("{host_info}");
        assert_eq!(
            xml,
            r#"<host_info>
    <tz_shift/>
    <domain_name/>
    <serialnum/>
    <ip_addr/>
    <host_cpid/>
    <p_ncpus/>
    <p_vendor/>
    <p_model/>
    <p_features/>
    <p_fpops/>
    <p_iops/>
    <p_membw/>
    <p_calculated/>
    <p_vm_extensions_disabled/>
    <m_nbytes/>
    <m_cache/>
    <m_swap/>
    <d_total/>
    <d_free/>
    <os_name/>
    <os_version/>
    <docker_version/>
    <docker_type/>
    <docker_compose_version/>
    <docker_compose_type/>
    <product_name/>
    <mac_address/>
    <virtualbox_version/>
    <num_opencl_cpu_platforms/>
</host_info>"#
        );
    }

    #[test]
    fn test_serialize_get_host_info() {
        let expected = r#"<get_host_info/>"#;

        let result = quick_xml::se::to_string(&GetHostInfo {}).unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    fn test_parse() {
        testing_logger::setup();

        let expected = HostInfo {
            tz_shift: Some(7),
            domain_name: Some("".to_string()),
            serialnum: Some("".to_string()),
            ip_addr: Some("".to_string()),
            host_cpid: Some("".to_string()),
            p_ncpus: Some(0),
            p_vendor: Some("".to_string()),
            p_model: Some("".to_string()),
            p_features: Some("".to_string()),
            p_fpops: Some(0.0),
            p_iops: Some(0.0),
            p_membw: Some(0.0),
            p_calculated: Some(0.0),
            p_vm_extensions_disabled: Some(true),
            m_nbytes: Some(0.0),
            m_cache: Some(0.0),
            m_swap: Some(0.0),
            d_total: Some(0.0),
            d_free: Some(0.0),
            os_name: Some("".to_string()),
            os_version: Some("".to_string()),
            docker_version: Some("".to_string()),
            docker_type: Some(DockerType::Podman),
            docker_compose_version: Some("".to_string()),
            docker_compose_type: Some(DockerType::Docker),
            product_name: Some("".to_string()),
            mac_address: Some("".to_string()),
            virtualbox_version: Some("".to_string()),
            num_opencl_cpu_platforms: Some(0),
        };

        let xml = r#"
<host_info>
    <tz_shift>7</tz_shift>
    <domain_name/>
    <serialnum/>
    <ip_addr/>
    <host_cpid/>
    <p_ncpus>0</p_ncpus>
    <p_vendor/>
    <p_model/>
    <p_features/>
    <p_fpops>0</p_fpops>
    <p_iops>0</p_iops>
    <p_membw>0</p_membw>
    <p_calculated>0</p_calculated>
    <p_vm_extensions_disabled>true</p_vm_extensions_disabled>
    <m_nbytes>0</m_nbytes>
    <m_cache>0</m_cache>
    <m_swap>0</m_swap>
    <d_total>0</d_total>
    <d_free>0</d_free>
    <os_name/>
    <os_version/>
    <docker_version/>
    <docker_type>podman</docker_type>
    <docker_compose_version/>
    <docker_compose_type>docker</docker_compose_type>
    <product_name/>
    <mac_address/>
    <virtualbox_version/>
    <num_opencl_cpu_platforms>0</num_opencl_cpu_platforms>
</host_info>
"#;

        let result: HostInfo = quick_xml::de::from_str(&xml).unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    fn test_unparse() {
        let host_info = HostInfo {
            tz_shift: Some(7),
            p_ncpus: Some(0),
            p_fpops: Some(0.0),
            p_iops: Some(0.0),
            p_membw: Some(0.0),
            p_calculated: Some(0.0),
            m_nbytes: Some(0.0),
            m_cache: Some(0.0),
            m_swap: Some(0.0),
            d_total: Some(0.0),
            d_free: Some(0.0),
            p_vm_extensions_disabled: Some(true),
            docker_type: Some(DockerType::Podman),
            docker_compose_type: Some(DockerType::Docker),
            num_opencl_cpu_platforms: Some(0),
            ..Default::default()
        };

        let expected = "\
<host_info>\
<tz_shift>7</tz_shift>\
<domain_name/>\
<serialnum/>\
<ip_addr/>\
<host_cpid/>\
<p_ncpus>0</p_ncpus>\
<p_vendor/>\
<p_model/>\
<p_features/>\
<p_fpops>0</p_fpops>\
<p_iops>0</p_iops>\
<p_membw>0</p_membw>\
<p_calculated>0</p_calculated>\
<p_vm_extensions_disabled>true</p_vm_extensions_disabled>\
<m_nbytes>0</m_nbytes>\
<m_cache>0</m_cache>\
<m_swap>0</m_swap>\
<d_total>0</d_total>\
<d_free>0</d_free>\
<os_name/>\
<os_version/>\
<docker_version/>\
<docker_type>podman</docker_type>\
<docker_compose_version/>\
<docker_compose_type>docker</docker_compose_type>\
<product_name/>\
<mac_address/>\
<virtualbox_version/>\
<num_opencl_cpu_platforms>0</num_opencl_cpu_platforms>\
</host_info>";

        let result = quick_xml::se::to_string(&host_info).unwrap();

        assert_eq!(expected, result);
    }
}
