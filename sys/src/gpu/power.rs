/// Undocumented API
pub mod private {
    use crate::prelude_::*;
    use crate::gpu::pstate::{VoltageInfoDomain, NV_GPU_PERF_VOLTAGE_INFO_DOMAIN_ID};
    use crate::gpu::clock::NVAPI_MAX_GPU_PERF_VOLTAGES;

    nvstruct! {
        pub struct NV_GPU_CLIENT_VOLT_RAILS_STATUS_V1 {
            pub version: NvVersion,
            pub flags: u32,
            pub zero: Padding<[u32; 8]>,
            pub value_uV: u32,
            pub unknown: Padding<[u32; 8]>,
        }
    }

    nvversion! { NV_GPU_CLIENT_VOLT_RAILS_STATUS(NvAPI_GPU_ClientVoltRailsGetStatus):
        NV_GPU_CLIENT_VOLT_RAILS_STATUS_V1(1) = 76
    }

    nvapi! {
        /// Pascal and later
        pub fn NvAPI_GPU_ClientVoltRailsGetStatus(hPhysicalGPU@self: NvPhysicalGpuHandle, pVoltageStatus@StructVersionOut: *mut NV_GPU_CLIENT_VOLT_RAILS_STATUS) -> NvAPI_Status;

        impl self {
            pub fn ClientVoltRailsGetStatus;
        }
    }

    nvstruct! {
        pub struct NV_GPU_CLIENT_VOLT_RAILS_CONTROL_V1 {
            pub version: NvVersion,
            /// uiDelta
            pub percent: u32, // apparently actually i32?
            pub unknown: Padding<[u32; 8]>,
        }
    }

    nvversion! { NV_GPU_CLIENT_VOLT_RAILS_CONTROL(NvAPI_GPU_ClientVoltRailsGetControl, NvAPI_GPU_ClientVoltRailsSetControl):
        NV_GPU_CLIENT_VOLT_RAILS_CONTROL_V1(1)
    }

    nvapi! {
        /// Pascal and later
        pub fn NvAPI_GPU_ClientVoltRailsGetControl(hPhysicalGPU@self: NvPhysicalGpuHandle, pVoltboostPercent@StructVersionOut: *mut NV_GPU_CLIENT_VOLT_RAILS_CONTROL) -> NvAPI_Status;

        impl self {
            pub fn ClientVoltRailsGetControl;
        }
    }

    nvapi! {
        /// Pascal and later
        pub fn NvAPI_GPU_ClientVoltRailsSetControl(hPhysicalGPU@self: NvPhysicalGpuHandle, pVoltboostPercent@StructVersion: *const NV_GPU_CLIENT_VOLT_RAILS_CONTROL) -> NvAPI_Status;

        impl self {
            pub fn ClientVoltRailsSetControl;
        }
    }

    nvstruct! {
        #[derive(Hash, PartialOrd, Ord, PartialEq, Eq, Default)]
        pub struct NV_GPU_CLOCK_CLIENT_CLK_VF_POINT {
            pub freq_kHz: u32,
            pub voltage_uV: u32,
        }
    }

    impl NV_GPU_CLOCK_CLIENT_CLK_VF_POINT {
        pub fn to_option(&self) -> Option<Self> {
            match self {
                Self { freq_kHz: 0, voltage_uV: 0 } => None,
                &point => Some(point)
            }
        }
    }

    nvstruct! {
        #[derive(Hash, PartialOrd, Ord, PartialEq, Eq, Default)]
        pub struct NV_GPU_CLOCK_CLIENT_CLK_VF_POINT_STATUS_V1 {
            pub clock_type: u32, // 0, 1 for idle mem values?
            pub point: NV_GPU_CLOCK_CLIENT_CLK_VF_POINT,
            pub unknown: Padding<[u32; 4]>,
        }
    }

    nvstruct! {
        #[derive(Default)]
        pub struct NV_GPU_CLOCK_CLIENT_CLK_VF_POINT_STATUS_V3 {
            pub clock_type: u32, // 0, 1?
            pub point: NV_GPU_CLOCK_CLIENT_CLK_VF_POINT,
            pub point_default: NV_GPU_CLOCK_CLIENT_CLK_VF_POINT,
            pub unknown0: Padding<[u32; 8]>,
            /// overclockedFrequencyKhz and millivoltage
            pub point_overclocked: NV_GPU_CLOCK_CLIENT_CLK_VF_POINT,
            pub unknown: Padding<[u32; 348/4 - (7 + 8)]>,
        }
    }

    impl NV_GPU_CLOCK_CLIENT_CLK_VF_POINT_STATUS_V3 {
        pub fn point_configured(&self) -> NV_GPU_CLOCK_CLIENT_CLK_VF_POINT {
            self.point_overclocked.to_option()
                .unwrap_or(self.point)
        }
    }

    nvstruct! {
        pub struct NV_GPU_CLOCK_CLIENT_CLK_VF_POINTS_STATUS_V1 {
            pub version: NvVersion,
            pub mask: ClockMask,
            pub unknown: Padding<[u32; 8]>,
            pub points: Array<[NV_GPU_CLOCK_CLIENT_CLK_VF_POINT_STATUS_V1; 255]>,
        }
    }

    impl NV_GPU_CLOCK_CLIENT_CLK_VF_POINTS_STATUS_V1 {
        pub fn points<'a>(&'a self) -> impl Iterator<Item=&'a NV_GPU_CLOCK_CLIENT_CLK_VF_POINT_STATUS_V1> + 'a {
            self.mask.iter()
                .map(|i| self.points.get(i).unwrap())
        }

        pub fn into_points(self) -> impl Iterator<Item=NV_GPU_CLOCK_CLIENT_CLK_VF_POINT_STATUS_V1> {
            let points = self.points;
            let mask: Vec<_> = self.mask.into_iter().collect();
            mask.into_iter()
                .map(move |i| *points.get(i).unwrap())
        }
    }

    nvstruct! {
        pub struct NV_GPU_CLOCK_CLIENT_CLK_VF_POINTS_STATUS_V3 {
            pub version: NvVersion,
            pub mask: ClockMask,
            pub unknown: Padding<[u8; 0x44]>,
            pub points: Array<[NV_GPU_CLOCK_CLIENT_CLK_VF_POINT_STATUS_V3; 255]>,
        }
    }

    impl NV_GPU_CLOCK_CLIENT_CLK_VF_POINTS_STATUS_V3 {
        pub fn points<'a>(&'a self) -> impl Iterator<Item=&'a NV_GPU_CLOCK_CLIENT_CLK_VF_POINT_STATUS_V3> + 'a {
            self.mask.iter()
                .map(|i| self.points.get(i).unwrap())
        }

        pub fn into_points(self) -> impl Iterator<Item=NV_GPU_CLOCK_CLIENT_CLK_VF_POINT_STATUS_V3> {
            let points = self.points;
            let mask: Vec<_> = self.mask.into_iter().collect();
            mask.into_iter()
                .map(move |i| *points.get(i).unwrap())
        }
    }

    nvversion! { NV_GPU_CLOCK_CLIENT_CLK_VF_POINTS_STATUS(NvAPI_GPU_ClockClientClkVfPointsGetStatus):
        NV_GPU_CLOCK_CLIENT_CLK_VF_POINTS_STATUS_V3(3) = 0x15b0c,
        NV_GPU_CLOCK_CLIENT_CLK_VF_POINTS_STATUS_V1(2) = 0x1c28,
        NV_GPU_CLOCK_CLIENT_CLK_VF_POINTS_STATUS_V1(1; @old) = 0x1c28
    }

    nvapi! {
        /// Pascal and later
        pub fn NvAPI_GPU_ClockClientClkVfPointsGetStatus(hPhysicalGPU@self: NvPhysicalGpuHandle, pVfpCurve@StructVersion: *mut NV_GPU_CLOCK_CLIENT_CLK_VF_POINTS_STATUS) -> NvAPI_Status;

        impl self {
            pub fn ClockClientClkVfPointsGetStatus;
        }
    }

    nvenum! {
        pub enum NV_GPU_CLIENT_POWER_POLICIES_POLICY_ID / PowerPolicyId {
            NV_GPU_CLIENT_POWER_POLICIES_POLICY_ID_DEFAULT / Default = 0,
        }
    }

    nvenum_display! {
        PowerPolicyId => {
            Default = "Board Power Limit",
        }
    }

    nvstruct! {
        #[derive(Default)]
        pub struct NV_GPU_CLIENT_POWER_POLICY_INFO_V1 {
            pub policy_id: NV_GPU_CLIENT_POWER_POLICIES_POLICY_ID,
            pub b: u32,
            pub c: u32,
            pub min_power: u32,
            pub e: u32,
            pub f: u32,
            pub def_power: u32,
            pub h: u32,
            pub i: u32,
            pub max_power: u32,
            pub k: u32, // 0
        }
    }

    nvtag! { NV_GPU_CLIENT_POWER_POLICY_INFO_V1.policy_id: NV_GPU_CLIENT_POWER_POLICIES_POLICY_ID / PowerPolicyId @TaggedData }

    nvstruct! {
        pub struct NV_GPU_CLIENT_POWER_POLICIES_INFO_V1 {
            pub version: NvVersion,
            pub valid: u8,
            pub count: u8,
            pub padding: Padding<[u8; 2]>,
            pub entries: Array<[NV_GPU_CLIENT_POWER_POLICY_INFO_V1; 4]>,
        }
    }

    nventries! { NV_GPU_CLIENT_POWER_POLICIES_INFO_V1.entries[..count]@(get_entries/set_entries/entries_mut):
        [NV_GPU_CLIENT_POWER_POLICY_INFO_V1; 4]
    }

    nvstruct! {
        #[derive(Default)]
        pub struct NV_GPU_CLIENT_POWER_POLICY_INFO_V2 {
            pub policy_id: NV_GPU_CLIENT_POWER_POLICIES_POLICY_ID,
            pub unknown0: Padding<[u32; 3]>,
            pub min_power: u32,
            pub unknown1: Padding<[u32; 2]>,
            pub def_power: u32,
            pub unknown2: Padding<[u32; 2]>,
            pub max_power: u32,
            pub padding: Padding<[u32; 560/4 - 11]>,
        }
    }

    nvtag! { NV_GPU_CLIENT_POWER_POLICY_INFO_V2.policy_id: NV_GPU_CLIENT_POWER_POLICIES_POLICY_ID / PowerPolicyId @TaggedData }

    nvstruct! {
        pub struct NV_GPU_CLIENT_POWER_POLICIES_INFO_V2 {
            pub version: NvVersion,
            pub valid: u8,
            pub count: u8,
            pub padding: Padding<[u8; 2]>,
            pub entries: Array<[NV_GPU_CLIENT_POWER_POLICY_INFO_V2; 4]>,
        }
    }

    nventries! { NV_GPU_CLIENT_POWER_POLICIES_INFO_V2.entries[..count]@(get_entries/set_entries/entries_mut):
        [NV_GPU_CLIENT_POWER_POLICY_INFO_V2; 4]
    }

    nvversion! { NV_GPU_CLIENT_POWER_POLICIES_INFO(NvAPI_GPU_ClientPowerPoliciesGetInfo):
        NV_GPU_CLIENT_POWER_POLICIES_INFO_V2(2) = 2248,
        NV_GPU_CLIENT_POWER_POLICIES_INFO_V1(1)
    }

    nvapi! {
        pub fn NvAPI_GPU_ClientPowerPoliciesGetInfo(hPhysicalGPU@self: NvPhysicalGpuHandle, pPowerInfo@StructVersionOut: *mut NV_GPU_CLIENT_POWER_POLICIES_INFO) -> NvAPI_Status;

        impl self {
            pub fn ClientPowerPoliciesGetInfo;
        }
    }

    nvstruct! {
        #[derive(Hash, PartialOrd, Ord, PartialEq, Eq, Default)]
        pub struct NV_GPU_CLIENT_POWER_POLICY_STATUS_V1 {
            pub policy_id: NV_GPU_CLIENT_POWER_POLICIES_POLICY_ID,
            pub b: u32,
            pub power_target: u32,
            pub d: u32,
        }
    }

    impl From<(NV_GPU_CLIENT_POWER_POLICIES_POLICY_ID, u32)> for NV_GPU_CLIENT_POWER_POLICY_STATUS_V1 {
        fn from((policy_id, power_target): (NV_GPU_CLIENT_POWER_POLICIES_POLICY_ID, u32)) -> Self {
            Self {
                policy_id,
                power_target,
                b: 0,
                d: 0,
            }
        }
    }

    impl From<NV_GPU_CLIENT_POWER_POLICY_STATUS_V2> for NV_GPU_CLIENT_POWER_POLICY_STATUS_V1 {
        #[inline]
        fn from(entry: NV_GPU_CLIENT_POWER_POLICY_STATUS_V2) -> Self {
            (entry.policy_id, entry.power_target).into()
        }
    }

    nvtag! { NV_GPU_CLIENT_POWER_POLICY_STATUS_V1.policy_id: NV_GPU_CLIENT_POWER_POLICIES_POLICY_ID / PowerPolicyId @TaggedData }

    nvstruct! {
        pub struct NV_GPU_CLIENT_POWER_POLICIES_STATUS_V1 {
            pub version: NvVersion,
            pub count: u32,
            pub entries: Array<[NV_GPU_CLIENT_POWER_POLICY_STATUS_V1; 4]>,
        }
    }

    nventries! { NV_GPU_CLIENT_POWER_POLICIES_STATUS_V1.entries[..count]@(get_entries/set_entries/entries_mut):
        [NV_GPU_CLIENT_POWER_POLICY_STATUS_V1; 4]
    }

    nvstruct! {
        #[derive(Default)]
        pub struct NV_GPU_CLIENT_POWER_POLICY_STATUS_V2 {
            pub policy_id: NV_GPU_CLIENT_POWER_POLICIES_POLICY_ID,
            pub unknown: Padding<[u32; 1]>,
            pub flags: u32,
            pub power_target: u32,
            pub padding: Padding<[u32; 340/4 - 4]>,
        }
    }

    impl From<(NV_GPU_CLIENT_POWER_POLICIES_POLICY_ID, u32)> for NV_GPU_CLIENT_POWER_POLICY_STATUS_V2 {
        fn from((policy_id, power_target): (NV_GPU_CLIENT_POWER_POLICIES_POLICY_ID, u32)) -> Self {
            Self {
                policy_id,
                power_target,
                flags: 0,
                unknown: Default::default(),
                padding: Default::default(),
            }
        }
    }

    impl From<NV_GPU_CLIENT_POWER_POLICY_STATUS_V1> for NV_GPU_CLIENT_POWER_POLICY_STATUS_V2 {
        #[inline]
        fn from(entry: NV_GPU_CLIENT_POWER_POLICY_STATUS_V1) -> Self {
            (entry.policy_id, entry.power_target).into()
        }
    }

    impl NV_GPU_CLIENT_POWER_POLICY_STATUS_V2 {
        /// Unsure what this is but flag should be cleared for SetStatus, maybe?
        pub fn set_flag(&mut self, value: bool) {
            self.flags = self.flags & 0xfffffffe | if value { 1 } else { 0 }
        }
    }

    nvtag! { NV_GPU_CLIENT_POWER_POLICY_STATUS_V2.policy_id: NV_GPU_CLIENT_POWER_POLICIES_POLICY_ID / PowerPolicyId @TaggedData }

    nvstruct! {
        pub struct NV_GPU_CLIENT_POWER_POLICIES_STATUS_V2 {
            pub version: NvVersion,
            pub count: u32,
            pub entries: Array<[NV_GPU_CLIENT_POWER_POLICY_STATUS_V2; 4]>,
        }
    }

    nventries! { NV_GPU_CLIENT_POWER_POLICIES_STATUS_V2.entries[..count]@(get_entries/set_entries/entries_mut):
        [NV_GPU_CLIENT_POWER_POLICY_STATUS_V2; 4]
    }

    nvversion! { NV_GPU_CLIENT_POWER_POLICIES_STATUS(NvAPI_GPU_ClientPowerPoliciesGetStatus, NvAPI_GPU_ClientPowerPoliciesSetStatus):
        NV_GPU_CLIENT_POWER_POLICIES_STATUS_V2(2) = 1368,
        NV_GPU_CLIENT_POWER_POLICIES_STATUS_V1(1)
    }

    nvapi! {
        pub fn NvAPI_GPU_ClientPowerPoliciesGetStatus(hPhysicalGPU@self: NvPhysicalGpuHandle, pPowerStatus@StructVersionOut: *mut NV_GPU_CLIENT_POWER_POLICIES_STATUS) -> NvAPI_Status;

        impl self {
            pub fn ClientPowerPoliciesGetStatus;
        }
    }

    nvapi! {
        pub fn NvAPI_GPU_ClientPowerPoliciesSetStatus(hPhysicalGPU@self: NvPhysicalGpuHandle, pPowerStatus@StructVersion: *const NV_GPU_CLIENT_POWER_POLICIES_STATUS) -> NvAPI_Status;

        impl self {
            pub fn ClientPowerPoliciesSetStatus;
        }
    }

    nvstruct! {
        pub struct NV_GPU_CLIENT_POWER_TOPOLOGY_INFO_V1 {
            pub version: NvVersion,
            pub valid: u8,
            pub count: u8,
            pub padding: Padding<[u8; 2]>,
            pub channels: Array<[NV_GPU_CLIENT_POWER_TOPOLOGY_CHANNEL_ID; 4]>,
        }
    }

    nventries! { NV_GPU_CLIENT_POWER_TOPOLOGY_INFO_V1.channels[..count]@(get_channels/set_channels/channels_mut):
        [NV_GPU_CLIENT_POWER_TOPOLOGY_CHANNEL_ID; 4]
    }

    nvversion! { NV_GPU_CLIENT_POWER_TOPOLOGY_INFO(NvAPI_GPU_ClientPowerTopologyGetInfo):
        NV_GPU_CLIENT_POWER_TOPOLOGY_INFO_V1(1) = 24
    }

    nvapi! {
        pub fn NvAPI_GPU_ClientPowerTopologyGetInfo(hPhysicalGPU@self: NvPhysicalGpuHandle, pPowerTopo@StructVersionOut: *mut NV_GPU_CLIENT_POWER_TOPOLOGY_INFO) -> NvAPI_Status;

        impl self {
            pub fn ClientPowerTopologyGetInfo;
        }
    }

    nvenum! {
        pub enum NV_GPU_CLIENT_POWER_TOPOLOGY_CHANNEL_ID / PowerTopologyChannelId {
            NV_GPU_CLIENT_POWER_TOPOLOGY_CHANNEL_ID_TOTAL_GPU_POWER / TotalGpuPower = 0,
            NV_GPU_CLIENT_POWER_TOPOLOGY_CHANNEL_ID_NORMALIZED_TOTAL_POWER / NormalizedTotalPower = 1,
        }
    }

    nvenum_display! {
        PowerTopologyChannelId => {
            TotalGpuPower = "Total Power",
            NormalizedTotalPower = "Normalized Power",
        }
    }

    nvstruct! {
        #[derive(Default)]
        pub struct NV_GPU_CLIENT_POWER_TOPOLOGY_STATUS_ENTRY {
            pub channel: NV_GPU_CLIENT_POWER_TOPOLOGY_CHANNEL_ID,
            pub unknown0: u32,
            pub power: u32,
            pub unknown1: u32,
        }
    }

    impl From<NV_GPU_CLIENT_POWER_TOPOLOGY_CHANNEL_ID> for NV_GPU_CLIENT_POWER_TOPOLOGY_STATUS_ENTRY {
        fn from(channel: NV_GPU_CLIENT_POWER_TOPOLOGY_CHANNEL_ID) -> Self {
            Self {
                channel,
                power: 0,
                unknown0: 0,
                unknown1: 0,
            }
        }
    }

    nvtag! { NV_GPU_CLIENT_POWER_TOPOLOGY_STATUS_ENTRY.channel: NV_GPU_CLIENT_POWER_TOPOLOGY_CHANNEL_ID / PowerTopologyChannelId @TaggedData }

    nvstruct! {
        pub struct NV_GPU_CLIENT_POWER_TOPOLOGY_STATUS_V1 {
            pub version: NvVersion,
            pub count: u32,
            pub entries: Array<[NV_GPU_CLIENT_POWER_TOPOLOGY_STATUS_ENTRY; 4]>,
        }
    }

    nventries! { NV_GPU_CLIENT_POWER_TOPOLOGY_STATUS_V1.entries[..count]@(get_entries/set_entries/entries_mut):
        [NV_GPU_CLIENT_POWER_TOPOLOGY_STATUS_ENTRY; 4]
    }

    nvversion! { NV_GPU_CLIENT_POWER_TOPOLOGY_STATUS(NvAPI_GPU_ClientPowerTopologyGetStatus):
        NV_GPU_CLIENT_POWER_TOPOLOGY_STATUS_V1(1) = 72
    }

    nvapi! {
        pub fn NvAPI_GPU_ClientPowerTopologyGetStatus(hPhysicalGPU@self: NvPhysicalGpuHandle, pPowerTopo@StructVersion: *mut NV_GPU_CLIENT_POWER_TOPOLOGY_STATUS) -> NvAPI_Status;

        impl self {
            pub fn ClientPowerTopologyGetStatus;
        }
    }

    nvbits! {
        pub enum NV_GPU_PERF_FLAGS / PerfFlags {
            NV_GPU_PERF_FLAGS_POWER_LIMIT / POWER_LIMIT = 1,
            NV_GPU_PERF_FLAGS_THERMAL_LIMIT / THERMAL_LIMIT = 2,
            /// Reliability voltage
            NV_GPU_PERF_FLAGS_VOLTAGE_REL_LIMIT / VOLTAGE_REL_LIMIT = 4,
            /// Operating voltage
            NV_GPU_PERF_FLAGS_VOLTAGE_OP_LIMIT / VOLTAGE_OP_LIMIT = 8,
            /// GPU utilization
            NV_GPU_PERF_FLAGS_NO_LOAD_LIMIT / NO_LOAD_LIMIT = 16,
            /// Never seen this
            NV_GPU_PERF_FLAGS_UNKNOWN_32 / UNKNOWN_32 = 32,
        }
    }

    nvenum_display! {
        PerfFlags => {
            POWER_LIMIT = "Power",
            THERMAL_LIMIT = "Temperature",
            VOLTAGE_REL_LIMIT = "Reliability Voltage",
            VOLTAGE_OP_LIMIT = "Operating Voltage",
            NO_LOAD_LIMIT = "No Load",
            UNKNOWN_32 = "Unknown32",
            _ = _,
        }
    }

    nvstruct! {
        pub struct NV_GPU_PERF_POLICIES_INFO_PARAMS_V1 {
            pub version: NvVersion,
            pub maxUnknown: u32,
            pub limitSupport: NV_GPU_PERF_FLAGS,
            pub padding: Padding<[u32; 16]>,
        }
    }

    nvversion! { NV_GPU_PERF_POLICIES_INFO_PARAMS(NvAPI_GPU_PerfPoliciesGetInfo):
        NV_GPU_PERF_POLICIES_INFO_PARAMS_V1(1) = 76
    }

    nvapi! {
        pub fn NvAPI_GPU_PerfPoliciesGetInfo(hPhysicalGPU@self: NvPhysicalGpuHandle, pPerfInfo@StructVersionOut: *mut NV_GPU_PERF_POLICIES_INFO_PARAMS) -> NvAPI_Status;

        impl self {
            pub fn PerfPoliciesGetInfo;
        }
    }

    nvstruct! {
        pub struct NV_GPU_PERF_POLICIES_STATUS_PARAMS_V1 {
            pub version: NvVersion,
            pub flags: u32,
            /// nanoseconds
            pub timer: u64,
            /// - 1 = power limit
            /// - 2 = temp limit
            /// - 4 = voltage limit
            /// - 8 = only got with 15 in driver crash
            /// - 16 = no-load limit
            pub limits: NV_GPU_PERF_FLAGS,
            pub zero0: u32,
            /// - 1 on load
            /// - 3 in low clocks
            /// - 7 in idle
            pub unknown: u32,
            pub zero1: u32,
            /// nanoseconds
            pub timers: [u64; 3],
            pub padding: Padding<[u32; 326]>,
        }
    }

    nvversion! { NV_GPU_PERF_POLICIES_STATUS_PARAMS(NvAPI_GPU_PerfPoliciesGetStatus):
        NV_GPU_PERF_POLICIES_STATUS_PARAMS_V1(1) = 0x550
    }

    nvapi! {
        pub fn NvAPI_GPU_PerfPoliciesGetStatus(hPhysicalGPU@self: NvPhysicalGpuHandle, pPerfStatus@StructVersionOut: *mut NV_GPU_PERF_POLICIES_STATUS_PARAMS) -> NvAPI_Status;

        impl self {
            pub fn PerfPoliciesGetStatus;
        }
    }

    nvstruct! {
        pub struct NV_VOLT_STATUS_V1 {
            pub version: NvVersion,
            pub flags: u32,
            /// unsure
            pub count: u32,
            pub unknown: u32,
            pub value_uV: u32,
            pub buf1: Padding<[u32; 30]>,
        }
    }

    nvversion! { NV_VOLT_STATUS(NvAPI_GPU_GetVoltageDomainsStatus/*, NvAPI_GPU_GetVoltageStep*/):
        NV_VOLT_STATUS_V1(1) = 140
    }

    nvapi! {
        /// Maxwell only
        pub fn NvAPI_GPU_GetVoltageDomainsStatus(hPhysicalGPU@self: NvPhysicalGpuHandle, pVoltStatus@StructVersionOut: *mut NV_VOLT_STATUS) -> NvAPI_Status;

        impl self {
            pub fn GetVoltageDomainsStatus;
        }
    }

    nvapi! {
        /// Maxwell only
        pub fn NvAPI_GPU_GetVoltageStep(hPhysicalGPU@self: NvPhysicalGpuHandle, pVoltStep@StructVersionOut: *mut NV_VOLT_STATUS) -> NvAPI_Status;

        impl self {
            pub fn GetVoltageStep;
        }
    }

    nvstruct! {
        #[derive(Default)]
        pub struct NV_VOLT_TABLE_ENTRY {
            pub voltage_domain: NV_GPU_PERF_VOLTAGE_INFO_DOMAIN_ID,
            pub voltage_uV: u32,
            pub unknown: Padding<[u32; 257]>,
        }
    }
    nvtag! { NV_VOLT_TABLE_ENTRY.voltage_domain@domain: NV_GPU_PERF_VOLTAGE_INFO_DOMAIN_ID / VoltageInfoDomain @TaggedData }

    nvstruct! {
        pub struct NV_VOLT_TABLE_V1 {
            pub version: NvVersion,
            pub flags: u32,
            pub count: u32,
            pub entries: Array<[NV_VOLT_TABLE_ENTRY; NVAPI_MAX_GPU_PERF_VOLTAGES]>,
        }
    }

    nventries! { NV_VOLT_TABLE_V1.entries[..count]@(get_entries/set_entries/entries_mut):
        [NV_VOLT_TABLE_ENTRY; NVAPI_MAX_GPU_PERF_VOLTAGES]
    }

    nvversion! { NV_VOLT_TABLE(NvAPI_GPU_GetVoltages):
        NV_VOLT_TABLE_V1(1) = 0x40cc
    }

    nvapi! {
        /// Maxwell only
        pub fn NvAPI_GPU_GetVoltages(hPhysicalGPU@self: NvPhysicalGpuHandle, pVolts@StructVersionOut: *mut NV_VOLT_TABLE) -> NvAPI_Status;

        impl self {
            pub fn GetVoltages;
        }
    }
}
