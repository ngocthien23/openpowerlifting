//! Defines the `Federation` field for the `meets` table.

#[derive(Deserialize, PartialEq)]
pub enum Federation {
    #[serde(rename = "365Strong")]
    _365Strong,
    AAPF,
    AAU,
    ADFPA,
    APA,
    APC,
    APF,
    AsianPF,
    BB,
    BPU,
    BP,
    CAPO,
    CommonwealthPF,
    CPF,
    CPL,
    CPU,
    EPA,
    EPF,
    FESUPO,
    FFForce,
    FPO,
    GPA,
    GPC,
    #[serde(rename = "GPC-GB")]
    GPCGB,
    #[serde(rename = "GPC-AUS")]
    GPCAUS,
    #[serde(rename = "GPC-NZ")]
    GPCNZ,
    HERC,
    IDFPF,
    IPA,
    IPF,
    IPL,
    IrishPF,
    MHP,
    MM,
    NAPF,
    NASA,
    NIPF,
    NPA,
    NSF,
    NZPF,
    OceaniaPF,
    ProRaw,
    PA,
    RAW,
    RAWU,
    RPS,
    RUPC,
    ScottishPL,
    SCT,
    SPF,
    THSPA,
    UPA,
    USAPL,
    USPF,
    USPA,
    WelshPA,
    WPC,
    WNPF,
    WRPF,
    #[serde(rename = "WRPF-AUS")]
    WRPFAUS,
    #[serde(rename = "WRPF-CAN")]
    WRPFCAN,
    WUAP,
    XPC,
}
