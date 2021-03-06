use std::os::raw::c_char;

use crate::{base::Uuid, expression::IExpressionList, type_list::ITypeList};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IFunctionDefinition_api {
    _unused: [u8; 0],
}
pub type IFunctionDefinition = *mut IFunctionDefinition_api;

extern "C" {
    pub fn IFunction_definition_release(s: IFunctionDefinition);
    pub fn IFunction_definition_retain(s: IFunctionDefinition);
    pub fn IFunction_definition_compare_iid(id: Uuid) -> bool;
    pub fn IFunction_definition_type_get_iid() -> Uuid;

    pub fn IFunction_definition_get_parameter_count(
        f: IFunctionDefinition,
    ) -> usize;
    pub fn IFunction_definition_get_parameter_types(
        f: IFunctionDefinition,
    ) -> ITypeList;
    pub fn IFunction_definition_get_defaults(
        f: IFunctionDefinition,
    ) -> IExpressionList;
    pub fn IFunction_definition_get_parameter_name(
        f: IFunctionDefinition,
        index: usize,
    ) -> *const c_char;
    pub fn IFunction_definition_get_parameter_index(
        f: IFunctionDefinition,
        name: *const c_char,
    ) -> usize;
    pub fn IFunction_definition_get_mdl_name(
        f: IFunctionDefinition,
    ) -> *const c_char;
    pub fn IFunction_definition_get_semantic(
        i: IFunctionDefinition,
    ) -> FunctionDefinitionSemantics;
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum FunctionDefinitionSemantics {
    Unknown = 0,
    ConvConstructor = 1,
    ElemConstructor = 2,
    ColorSpectrumConstructor = 3,
    MatrixElemConstructor = 4,
    MatrixDiagConstructor = 5,
    InvalidRefConstructor = 6,
    DefaultStructConstructor = 7,
    TextureConstructor = 8,
    ConvOperator = 9,
    OperatorFirst = 512,
    LogicalNot = 513,
    Positive = 514,
    Negative = 515,
    PreIncrement = 516,
    PreDecrement = 517,
    PostIncrement = 518,
    PostDecrement = 519,
    Cast = 520,
    BinaryFirst = 521,
    ArrayIndex = 522,
    Multiply = 523,
    Divide = 524,
    Modulo = 525,
    Plus = 526,
    Minus = 527,
    ShiftLeft = 528,
    ShiftRight = 529,
    UnsignedShiftRight = 530,
    Less = 531,
    LessOrEqual = 532,
    GreaterOrEqual = 533,
    Greater = 534,
    Equal = 535,
    NotEqual = 536,
    BitwiseAnd = 537,
    BitwiseXor = 538,
    BitwiseOr = 539,
    LogicalAnd = 540,
    LogicalOr = 541,
    Assign = 542,
    MultiplyAssign = 543,
    DivideAssign = 544,
    ModuloAssign = 545,
    PlusAssign = 546,
    MinusAssign = 547,
    ShiftLeftAssign = 548,
    ShiftRightAssign = 549,
    UnsignedShiftRightAssign = 550,
    BitwiseOrAssign = 551,
    BitwiseXorAssign = 552,
    BitwiseAndAssign = 553,
    Sequence = 554,
    Ternary = 555,
    IntrinsicMathFirst = 768,
    IntrinsicMathAcos = 769,
    IntrinsicMathAll = 770,
    IntrinsicMathAny = 771,
    IntrinsicMathAsin = 772,
    IntrinsicMathAtan = 773,
    IntrinsicMathAtan2 = 774,
    IntrinsicMathAverage = 775,
    IntrinsicMathCeil = 776,
    IntrinsicMathClamp = 777,
    IntrinsicMathCos = 778,
    IntrinsicMathCross = 779,
    IntrinsicMathDegrees = 780,
    IntrinsicMathDistance = 781,
    IntrinsicMathDot = 782,
    IntrinsicMathEvalAtWavelength = 783,
    IntrinsicMathExp = 784,
    IntrinsicMathExp2 = 785,
    IntrinsicMathFloor = 786,
    IntrinsicMathFmod = 787,
    IntrinsicMathFrac = 788,
    IntrinsicMathIsnan = 789,
    IntrinsicMathIsfinite = 790,
    IntrinsicMathLength = 791,
    IntrinsicMathLerp = 792,
    IntrinsicMathLog = 793,
    IntrinsicMathLog2 = 794,
    IntrinsicMathLog10 = 795,
    IntrinsicMathLuminance = 796,
    IntrinsicMathMax = 797,
    IntrinsicMathMaxValue = 798,
    IntrinsicMathMaxValueWavelength = 799,
    IntrinsicMathMin = 800,
    IntrinsicMathMinValue = 801,
    IntrinsicMathMinValueWavelength = 802,
    IntrinsicMathModf = 803,
    IntrinsicMathNormalize = 804,
    IntrinsicMathPow = 805,
    IntrinsicMathRadians = 806,
    IntrinsicMathRound = 807,
    IntrinsicMathRsqrt = 808,
    IntrinsicMathSaturate = 809,
    IntrinsicMathSign = 810,
    IntrinsicMathSin = 811,
    IntrinsicMathSincos = 812,
    IntrinsicMathSmoothstep = 813,
    IntrinsicMathSqrt = 814,
    IntrinsicMathStep = 815,
    IntrinsicMathTan = 816,
    IntrinsicMathTranspose = 817,
    IntrinsicMathBlackbody = 818,
    IntrinsicMathEmissionColor = 819,
    IntrinsicMathDx = 820,
    IntrinsicMathDy = 821,
    IntrinsicStateFirst = 1024,
    IntrinsicStateNormal = 1025,
    IntrinsicStateGeometryNormal = 1026,
    IntrinsicStateMotion = 1027,
    IntrinsicStateTextureSpaceMax = 1028,
    IntrinsicStateTextureCoordinate = 1029,
    IntrinsicStateTextureTangentU = 1030,
    IntrinsicStateTextureTangentV = 1031,
    IntrinsicStateTangentSpace = 1032,
    IntrinsicStateGeometryTangentU = 1033,
    IntrinsicStateGeometryTangentV = 1034,
    IntrinsicStateDirection = 1035,
    IntrinsicStateAnimationTime = 1036,
    IntrinsicStateWavelengthBase = 1037,
    IntrinsicStateTransform = 1038,
    IntrinsicStateTransformPoint = 1039,
    IntrinsicStateTransformVector = 1040,
    IntrinsicStateTransformNormal = 1041,
    IntrinsicStateTransformScale = 1042,
    IntrinsicStateRoundedCornerNormal = 1043,
    IntrinsicStateMetersPerSceneUnit = 1044,
    IntrinsicStateSceneUnitsPerMeter = 1045,
    IntrinsicStateObjectId = 1046,
    IntrinsicStateWavelengthMin = 1047,
    IntrinsicStateWavelengthMax = 1048,
    IntrinsicTexFirst = 1280,
    IntrinsicTexHeight = 1281,
    IntrinsicTexDepth = 1282,
    IntrinsicTexLookupFloat = 1283,
    IntrinsicTexLookupFloat2 = 1284,
    IntrinsicTexLookupFloat3 = 1285,
    IntrinsicTexLookupFloat4 = 1286,
    IntrinsicTexLookupColor = 1287,
    IntrinsicTexTexelFloat = 1288,
    IntrinsicTexTexelFloat2 = 1289,
    IntrinsicTexTexelFloat3 = 1290,
    IntrinsicTexTexelFloat4 = 1291,
    IntrinsicTexTexelColor = 1292,
    IntrinsicTexTextureIsvalid = 1293,
    IntrinsicDfDiffuseReflectionBsdf = 1536,
    IntrinsicDfDiffuseTransmissionBsdf = 1537,
    IntrinsicDfSpecularBsdf = 1538,
    IntrinsicDfSimpleGlossyBsdf = 1539,
    IntrinsicDfBackscatteringGlossyReflectionBsdf = 1540,
    IntrinsicDfMeasuredBsdf = 1541,
    IntrinsicDfDiffuseEdf = 1542,
    IntrinsicDfMeasuredEdf = 1543,
    IntrinsicDfSpotEdf = 1544,
    IntrinsicDfAnisotropicVdf = 1545,
    IntrinsicDfNormalizedMix = 1546,
    IntrinsicDfClampedMix = 1547,
    IntrinsicDfWeightedLayer = 1548,
    IntrinsicDfFresnelLayer = 1549,
    IntrinsicDfCustomCurveLayer = 1550,
    IntrinsicDfMeasuredCurveLayer = 1551,
    IntrinsicDfThinFilm = 1552,
    IntrinsicDfTint = 1553,
    IntrinsicDfDirectionalFactor = 1554,
    IntrinsicDfMeasuredCurveFactor = 1555,
    IntrinsicDfLightProfilePower = 1556,
    IntrinsicDfLightProfileMaximum = 1557,
    IntrinsicDfLightProfileIsvalid = 1558,
    IntrinsicDfBsdfMeasurementIsvalid = 1559,
    IntrinsicDfMicrofacetBeckmannSmithBsdf = 1560,
    IntrinsicDfMicrofacetGgxSmithBsdf = 1561,
    IntrinsicDfMicrofacetBeckmannVcavitiesBsdf = 1562,
    IntrinsicDfMicrofacetGgxVcavitiesBsdf = 1563,
    IntrinsicDfWardGeislerMoroderBsdf = 1564,
    IntrinsicDfColorNormalizedMix = 1565,
    IntrinsicDfColorClampedMix = 1566,
    IntrinsicDfColorWeightedLayer = 1567,
    IntrinsicDfColorFresnelLayer = 1568,
    IntrinsicDfColorCustomCurveLayer = 1569,
    IntrinsicDfColorMeasuredCurveLayer = 1570,
    IntrinsicDfFresnelFactor = 1571,
    IntrinsicDfMeasuredFactor = 1572,
    IntrinsicDfChiangHairBsdf = 1573,
    IntrinsicNvidiaDfFirst = 1792,
    IntrinsicNvidiaDfWardGmGlossyBsdf = 1793,
    IntrinsicNvidiaDfMicrofacetBeckmannSmithBsdf = 1794,
    IntrinsicNvidiaDfMicrofacetGgxSmithBsdf = 1795,
    IntrinsicNvidiaDfMicrofacetBeckmannVcBsdf = 1796,
    IntrinsicNvidiaDfMicrofacetGgxVcBsdf = 1797,
    IntrinsicNvidiaDfMicrofacetPhongVcBsdf = 1798,
    IntrinsicNvidiaDfSimpleGlossyBsdf = 1799,
    IntrinsicNvidiaDfSimpleGlossyBsdfLegacy = 1800,
    IntrinsicNvidiaDfLegacyMcpGlossyBsdf = 1801,
    IntrinsicNvidiaDfMicrofacetSinKSmithBsdf = 1802,
    IntrinsicNvidiaDfMicrofacetSinKVcBsdf = 1803,
    IntrinsicDebugFirst = 2048,
    IntrinsicDebugAssert = 2049,
    IntrinsicDebugPrint = 2050,
    IntrinsicDagFirst = 2304,
    IntrinsicDagArrayConstructor = 2305,
    IntrinsicDagIndexAccess = 2306,
    IntrinsicDagArrayLength = 2307,
}
