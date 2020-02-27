#![doc = "Generated for FIT SDK profile version: "]
#![doc = "20.66.00"]
use byteorder::ByteOrder;
use error;
use profile;
use types;
#[doc = r" The actual data of a `Message`."]
#[derive(Debug,Clone)]
pub struct Field<T> {
    pub raw_value: T,
    pub scale:     Option<f64>,
    pub offset:    Option<f64>,
    pub units: Option<&'static str>,
}

impl<T: profile::base::Valid> Field<T> {
    pub fn is_valid(&self) -> bool {
        self.raw_value.is_valid()
    }
}

impl types::field::Field for Field<profile::base::Float32> {
    type Value = f64;

    fn value(&self) -> Self::Value {
        self.raw_value.0 as f64 / self.scale.unwrap_or(1.0) - self.offset.unwrap_or(0.0)
    }
}

impl types::field::Field for Field<profile::base::Float64> {
    type Value = f64;

    fn value(&self) -> Self::Value {
        self.raw_value.0 / self.scale.unwrap_or(1.0) - self.offset.unwrap_or(0.0)
    }
}

impl types::field::Field for Field<profile::base::Uint8> {
    type Value = f64;

    fn value(&self) -> Self::Value  {
        self.raw_value.0 as f64 / self.scale.unwrap_or(1.0) - self.offset.unwrap_or(0.0)
    }
}

impl types::field::Field for Field<profile::base::Uint8z> {
    type Value = f64;

    fn value(&self) -> Self::Value  {
        self.raw_value.0 as f64 / self.scale.unwrap_or(1.0) - self.offset.unwrap_or(0.0)
    }
}

impl types::field::Field for Field<profile::base::Sint8> {
    type Value = f64;

    fn value(&self) -> Self::Value {
        self.raw_value.0 as f64 / self.scale.unwrap_or(1.0) - self.offset.unwrap_or(0.0)
    }
}

impl types::field::Field for Field<profile::base::Uint16> {
    type Value = f64;

    fn value(&self) -> Self::Value  {
        self.raw_value.0 as f64 / self.scale.unwrap_or(1.0) - self.offset.unwrap_or(0.0)
    }
}

impl types::field::Field for Field<profile::base::Uint16z> {
    type Value = f64;

    fn value(&self) -> Self::Value  {
        self.raw_value.0 as f64 / self.scale.unwrap_or(1.0) - self.offset.unwrap_or(0.0)
    }
}

impl types::field::Field for Field<profile::base::Sint16> {
    type Value = f64;

    fn value(&self) -> Self::Value {
        self.raw_value.0 as f64 / self.scale.unwrap_or(1.0) - self.offset.unwrap_or(0.0)
    }
}

impl types::field::Field for Field<profile::base::Uint32> {
    type Value = f64;

    fn value(&self) -> Self::Value  {
        self.raw_value.0 as f64 / self.scale.unwrap_or(1.0) - self.offset.unwrap_or(0.0)
    }
}

impl types::field::Field for Field<profile::base::Uint32z> {
    type Value = f64;

    fn value(&self) -> Self::Value  {
        self.raw_value.0 as f64 / self.scale.unwrap_or(1.0) - self.offset.unwrap_or(0.0)
    }
}

impl types::field::Field for Field<profile::base::Sint32> {
    type Value = f64;

    fn value(&self) -> Self::Value {
        self.raw_value.0 as f64 / self.scale.unwrap_or(1.0) - self.offset.unwrap_or(0.0)
    }
}

impl types::field::Field for Field<profile::base::Uint64> {
    type Value = f64;

    fn value(&self) -> Self::Value  {
        self.raw_value.0 as f64 / self.scale.unwrap_or(1.0) - self.offset.unwrap_or(0.0)
    }
}

impl types::field::Field for Field<profile::base::Uint64z> {
    type Value = f64;

    fn value(&self) -> Self::Value  {
        self.raw_value.0 as f64 / self.scale.unwrap_or(1.0) - self.offset.unwrap_or(0.0)
    }
}

impl types::field::Field for Field<profile::base::Sint64> {
    type Value = f64;

    fn value(&self) -> Self::Value {
        self.raw_value.0 as f64 / self.scale.unwrap_or(1.0) - self.offset.unwrap_or(0.0)
    }
}

#[doc = r" All the FIT message types."]
#[derive(Debug,Clone)]
pub enum Message {
    FileId(FileId),
    FileCreator(FileCreator),
    TimestampCorrelation(TimestampCorrelation),
    Software(Software),
    SlaveDevice(SlaveDevice),
    Capabilities(Capabilities),
    FileCapabilities(FileCapabilities),
    MesgCapabilities(MesgCapabilities),
    FieldCapabilities(FieldCapabilities),
    DeviceSettings(DeviceSettings),
    UserProfile(UserProfile),
    HrmProfile(HrmProfile),
    SdmProfile(SdmProfile),
    BikeProfile(BikeProfile),
    Connectivity(Connectivity),
    WatchfaceSettings(WatchfaceSettings),
    OhrSettings(OhrSettings),
    ZonesTarget(ZonesTarget),
    Sport(Sport),
    HrZone(HrZone),
    SpeedZone(SpeedZone),
    CadenceZone(CadenceZone),
    PowerZone(PowerZone),
    MetZone(MetZone),
    DiveSettings(DiveSettings),
    DiveAlarm(DiveAlarm),
    DiveGas(DiveGas),
    Goal(Goal),
    Activity(Activity),
    Session(Session),
    Lap(Lap),
    Length(Length),
    Record(Record),
    Event(Event),
    DeviceInfo(DeviceInfo),
    TrainingFile(TrainingFile),
    Hrv(Hrv),
    WeatherConditions(WeatherConditions),
    WeatherAlert(WeatherAlert),
    GpsMetadata(GpsMetadata),
    CameraEvent(CameraEvent),
    GyroscopeData(GyroscopeData),
    AccelerometerData(AccelerometerData),
    MagnetometerData(MagnetometerData),
    BarometerData(BarometerData),
    ThreeDSensorCalibration(ThreeDSensorCalibration),
    OneDSensorCalibration(OneDSensorCalibration),
    VideoFrame(VideoFrame),
    ObdiiData(ObdiiData),
    NmeaSentence(NmeaSentence),
    AviationAttitude(AviationAttitude),
    Video(Video),
    VideoTitle(VideoTitle),
    VideoDescription(VideoDescription),
    VideoClip(VideoClip),
    Set(Set),
    Course(Course),
    CoursePoint(CoursePoint),
    SegmentId(SegmentId),
    SegmentLeaderboardEntry(SegmentLeaderboardEntry),
    SegmentPoint(SegmentPoint),
    SegmentLap(SegmentLap),
    SegmentFile(SegmentFile),
    Workout(Workout),
    WorkoutSession(WorkoutSession),
    WorkoutStep(WorkoutStep),
    ExerciseTitle(ExerciseTitle),
    Schedule(Schedule),
    Totals(Totals),
    WeightScale(WeightScale),
    BloodPressure(BloodPressure),
    MonitoringInfo(MonitoringInfo),
    Monitoring(Monitoring),
    Hr(Hr),
    StressLevel(StressLevel),
    MemoGlob(MemoGlob),
    AntChannelId(AntChannelId),
    AntRx(AntRx),
    AntTx(AntTx),
    ExdScreenConfiguration(ExdScreenConfiguration),
    ExdDataFieldConfiguration(ExdDataFieldConfiguration),
    ExdDataConceptConfiguration(ExdDataConceptConfiguration),
    FieldDescription(FieldDescription),
    DeveloperDataId(DeveloperDataId),
    DiveSummary(DiveSummary),
    Unknown { data:          Vec<u8>, mesg_num:      u16, field_def_num: u8 },
}
impl Message {
    pub(crate) fn decode<T: ByteOrder>(
        buffer: &[u8],
        mesg_num: u16,
        field_def_num: u8,
    ) -> error::Result<Self> {
        match mesg_num {
            0 => {
                FileId::decode::<T>(buffer, field_def_num).map(Message::FileId)
            },
            49 => {
                FileCreator::decode::<T>(buffer, field_def_num)
                    .map(Message::FileCreator)
            },
            162 => {
                TimestampCorrelation::decode::<T>(buffer, field_def_num)
                    .map(Message::TimestampCorrelation)
            },
            35 => {
                Software::decode::<T>(buffer, field_def_num)
                    .map(Message::Software)
            },
            106 => {
                SlaveDevice::decode::<T>(buffer, field_def_num)
                    .map(Message::SlaveDevice)
            },
            1 => {
                Capabilities::decode::<T>(buffer, field_def_num)
                    .map(Message::Capabilities)
            },
            37 => {
                FileCapabilities::decode::<T>(buffer, field_def_num)
                    .map(Message::FileCapabilities)
            },
            38 => {
                MesgCapabilities::decode::<T>(buffer, field_def_num)
                    .map(Message::MesgCapabilities)
            },
            39 => {
                FieldCapabilities::decode::<T>(buffer, field_def_num)
                    .map(Message::FieldCapabilities)
            },
            2 => {
                DeviceSettings::decode::<T>(buffer, field_def_num)
                    .map(Message::DeviceSettings)
            },
            3 => {
                UserProfile::decode::<T>(buffer, field_def_num)
                    .map(Message::UserProfile)
            },
            4 => {
                HrmProfile::decode::<T>(buffer, field_def_num)
                    .map(Message::HrmProfile)
            },
            5 => {
                SdmProfile::decode::<T>(buffer, field_def_num)
                    .map(Message::SdmProfile)
            },
            6 => {
                BikeProfile::decode::<T>(buffer, field_def_num)
                    .map(Message::BikeProfile)
            },
            127 => {
                Connectivity::decode::<T>(buffer, field_def_num)
                    .map(Message::Connectivity)
            },
            159 => {
                WatchfaceSettings::decode::<T>(buffer, field_def_num)
                    .map(Message::WatchfaceSettings)
            },
            188 => {
                OhrSettings::decode::<T>(buffer, field_def_num)
                    .map(Message::OhrSettings)
            },
            7 => {
                ZonesTarget::decode::<T>(buffer, field_def_num)
                    .map(Message::ZonesTarget)
            },
            12 => Sport::decode::<T>(buffer, field_def_num).map(Message::Sport),
            8 => {
                HrZone::decode::<T>(buffer, field_def_num).map(Message::HrZone)
            },
            53 => {
                SpeedZone::decode::<T>(buffer, field_def_num)
                    .map(Message::SpeedZone)
            },
            131 => {
                CadenceZone::decode::<T>(buffer, field_def_num)
                    .map(Message::CadenceZone)
            },
            9 => {
                PowerZone::decode::<T>(buffer, field_def_num)
                    .map(Message::PowerZone)
            },
            10 => {
                MetZone::decode::<T>(buffer, field_def_num)
                    .map(Message::MetZone)
            },
            258 => {
                DiveSettings::decode::<T>(buffer, field_def_num)
                    .map(Message::DiveSettings)
            },
            262 => {
                DiveAlarm::decode::<T>(buffer, field_def_num)
                    .map(Message::DiveAlarm)
            },
            259 => {
                DiveGas::decode::<T>(buffer, field_def_num)
                    .map(Message::DiveGas)
            },
            15 => Goal::decode::<T>(buffer, field_def_num).map(Message::Goal),
            34 => {
                Activity::decode::<T>(buffer, field_def_num)
                    .map(Message::Activity)
            },
            18 => {
                Session::decode::<T>(buffer, field_def_num)
                    .map(Message::Session)
            },
            19 => Lap::decode::<T>(buffer, field_def_num).map(Message::Lap),
            101 => {
                Length::decode::<T>(buffer, field_def_num).map(Message::Length)
            },
            20 => {
                Record::decode::<T>(buffer, field_def_num).map(Message::Record)
            },
            21 => Event::decode::<T>(buffer, field_def_num).map(Message::Event),
            23 => {
                DeviceInfo::decode::<T>(buffer, field_def_num)
                    .map(Message::DeviceInfo)
            },
            72 => {
                TrainingFile::decode::<T>(buffer, field_def_num)
                    .map(Message::TrainingFile)
            },
            78 => Hrv::decode::<T>(buffer, field_def_num).map(Message::Hrv),
            128 => {
                WeatherConditions::decode::<T>(buffer, field_def_num)
                    .map(Message::WeatherConditions)
            },
            129 => {
                WeatherAlert::decode::<T>(buffer, field_def_num)
                    .map(Message::WeatherAlert)
            },
            160 => {
                GpsMetadata::decode::<T>(buffer, field_def_num)
                    .map(Message::GpsMetadata)
            },
            161 => {
                CameraEvent::decode::<T>(buffer, field_def_num)
                    .map(Message::CameraEvent)
            },
            164 => {
                GyroscopeData::decode::<T>(buffer, field_def_num)
                    .map(Message::GyroscopeData)
            },
            165 => {
                AccelerometerData::decode::<T>(buffer, field_def_num)
                    .map(Message::AccelerometerData)
            },
            208 => {
                MagnetometerData::decode::<T>(buffer, field_def_num)
                    .map(Message::MagnetometerData)
            },
            209 => {
                BarometerData::decode::<T>(buffer, field_def_num)
                    .map(Message::BarometerData)
            },
            167 => {
                ThreeDSensorCalibration::decode::<T>(buffer, field_def_num)
                    .map(Message::ThreeDSensorCalibration)
            },
            210 => {
                OneDSensorCalibration::decode::<T>(buffer, field_def_num)
                    .map(Message::OneDSensorCalibration)
            },
            169 => {
                VideoFrame::decode::<T>(buffer, field_def_num)
                    .map(Message::VideoFrame)
            },
            174 => {
                ObdiiData::decode::<T>(buffer, field_def_num)
                    .map(Message::ObdiiData)
            },
            177 => {
                NmeaSentence::decode::<T>(buffer, field_def_num)
                    .map(Message::NmeaSentence)
            },
            178 => {
                AviationAttitude::decode::<T>(buffer, field_def_num)
                    .map(Message::AviationAttitude)
            },
            184 => {
                Video::decode::<T>(buffer, field_def_num).map(Message::Video)
            },
            185 => {
                VideoTitle::decode::<T>(buffer, field_def_num)
                    .map(Message::VideoTitle)
            },
            186 => {
                VideoDescription::decode::<T>(buffer, field_def_num)
                    .map(Message::VideoDescription)
            },
            187 => {
                VideoClip::decode::<T>(buffer, field_def_num)
                    .map(Message::VideoClip)
            },
            225 => Set::decode::<T>(buffer, field_def_num).map(Message::Set),
            31 => {
                Course::decode::<T>(buffer, field_def_num).map(Message::Course)
            },
            32 => {
                CoursePoint::decode::<T>(buffer, field_def_num)
                    .map(Message::CoursePoint)
            },
            148 => {
                SegmentId::decode::<T>(buffer, field_def_num)
                    .map(Message::SegmentId)
            },
            149 => {
                SegmentLeaderboardEntry::decode::<T>(buffer, field_def_num)
                    .map(Message::SegmentLeaderboardEntry)
            },
            150 => {
                SegmentPoint::decode::<T>(buffer, field_def_num)
                    .map(Message::SegmentPoint)
            },
            142 => {
                SegmentLap::decode::<T>(buffer, field_def_num)
                    .map(Message::SegmentLap)
            },
            151 => {
                SegmentFile::decode::<T>(buffer, field_def_num)
                    .map(Message::SegmentFile)
            },
            26 => {
                Workout::decode::<T>(buffer, field_def_num)
                    .map(Message::Workout)
            },
            158 => {
                WorkoutSession::decode::<T>(buffer, field_def_num)
                    .map(Message::WorkoutSession)
            },
            27 => {
                WorkoutStep::decode::<T>(buffer, field_def_num)
                    .map(Message::WorkoutStep)
            },
            264 => {
                ExerciseTitle::decode::<T>(buffer, field_def_num)
                    .map(Message::ExerciseTitle)
            },
            28 => {
                Schedule::decode::<T>(buffer, field_def_num)
                    .map(Message::Schedule)
            },
            33 => {
                Totals::decode::<T>(buffer, field_def_num).map(Message::Totals)
            },
            30 => {
                WeightScale::decode::<T>(buffer, field_def_num)
                    .map(Message::WeightScale)
            },
            51 => {
                BloodPressure::decode::<T>(buffer, field_def_num)
                    .map(Message::BloodPressure)
            },
            103 => {
                MonitoringInfo::decode::<T>(buffer, field_def_num)
                    .map(Message::MonitoringInfo)
            },
            55 => {
                Monitoring::decode::<T>(buffer, field_def_num)
                    .map(Message::Monitoring)
            },
            132 => Hr::decode::<T>(buffer, field_def_num).map(Message::Hr),
            227 => {
                StressLevel::decode::<T>(buffer, field_def_num)
                    .map(Message::StressLevel)
            },
            145 => {
                MemoGlob::decode::<T>(buffer, field_def_num)
                    .map(Message::MemoGlob)
            },
            82 => {
                AntChannelId::decode::<T>(buffer, field_def_num)
                    .map(Message::AntChannelId)
            },
            80 => AntRx::decode::<T>(buffer, field_def_num).map(Message::AntRx),
            81 => AntTx::decode::<T>(buffer, field_def_num).map(Message::AntTx),
            200 => {
                ExdScreenConfiguration::decode::<T>(buffer, field_def_num)
                    .map(Message::ExdScreenConfiguration)
            },
            201 => {
                ExdDataFieldConfiguration::decode::<T>(buffer, field_def_num)
                    .map(Message::ExdDataFieldConfiguration)
            },
            202 => {
                ExdDataConceptConfiguration::decode::<T>(buffer, field_def_num)
                    .map(Message::ExdDataConceptConfiguration)
            },
            206 => {
                FieldDescription::decode::<T>(buffer, field_def_num)
                    .map(Message::FieldDescription)
            },
            207 => {
                DeveloperDataId::decode::<T>(buffer, field_def_num)
                    .map(Message::DeveloperDataId)
            },
            268 => {
                DiveSummary::decode::<T>(buffer, field_def_num)
                    .map(Message::DiveSummary)
            },
            _ => {
                Ok(Message::Unknown {
                    data: buffer.to_vec(),
                    mesg_num,
                    field_def_num,
                })
            },
        }
    }
}
#[doc = "Must be first message in file."]
#[derive(Debug,Clone)]
pub enum FileId {
    Type(Field<profile::types::File>),
    Manufacturer(Field<profile::types::Manufacturer>),
    Product(Field<profile::base::Uint16>),
    SerialNumber(Field<profile::base::Uint32z>),
    #[doc = "Only set for files that are can be created/erased."]
    TimeCreated(Field<profile::types::DateTime>),
    #[doc = "Only set for files that are not created/erased."]
    Number(Field<profile::base::Uint16>),
    #[doc = "Optional free form string to indicate the devices name or model"]
    ProductName(Field<profile::base::Utf8String>),
    Unknown {
        data:          Vec<u8>,
        field_def_num: u8,
    },
}
impl FileId {
    pub(crate) fn decode<T: ByteOrder>(
        buffer: &[u8],
        field_def_num: u8,
    ) -> error::Result<Self> {
        match field_def_num {
            0 => {
                Ok(FileId::Type(Field {
                    raw_value:  profile::types::File::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            1 => {
                Ok(FileId::Manufacturer(Field {
                    raw_value:  profile::types::Manufacturer::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            2 => {
                Ok(FileId::Product(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            3 => {
                Ok(FileId::SerialNumber(Field {
                    raw_value:  profile::base::Uint32z::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            4 => {
                Ok(FileId::TimeCreated(Field {
                    raw_value:  profile::types::DateTime::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            5 => {
                Ok(FileId::Number(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            8 => {
                Ok(FileId::ProductName(Field {
                    raw_value:  profile::base::Utf8String::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            _ => {
                Ok(FileId::Unknown {
                    data: buffer.to_vec(),
                    field_def_num,
                })
            },
        }
    }
}
#[derive(Debug,Clone)]
pub enum FileCreator {
    SoftwareVersion(Field<profile::base::Uint16>),
    HardwareVersion(Field<profile::base::Uint8>),
    Unknown { data:          Vec<u8>, field_def_num: u8 },
}
impl FileCreator {
    pub(crate) fn decode<T: ByteOrder>(
        buffer: &[u8],
        field_def_num: u8,
    ) -> error::Result<Self> {
        match field_def_num {
            0 => {
                Ok(FileCreator::SoftwareVersion(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            1 => {
                Ok(FileCreator::HardwareVersion(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            _ => {
                Ok(FileCreator::Unknown {
                    data: buffer.to_vec(),
                    field_def_num,
                })
            },
        }
    }
}
#[derive(Debug,Clone)]
pub enum TimestampCorrelation {
    #[doc = "Whole second part of UTC timestamp at the time the system \
             timestamp was recorded."]
    Timestamp(Field<profile::types::DateTime>),
    #[doc = "Fractional part of the UTC timestamp at the time the system \
             timestamp was recorded."]
    FractionalTimestamp(Field<profile::base::Uint16>),
    #[doc = "Whole second part of the system timestamp"]
    SystemTimestamp(Field<profile::types::DateTime>),
    #[doc = "Fractional part of the system timestamp"]
    FractionalSystemTimestamp(Field<profile::base::Uint16>),
    #[doc = "timestamp epoch expressed in local time used to convert \
             timestamps to local time"]
    LocalTimestamp(Field<profile::types::LocalDateTime>),
    #[doc = "Millisecond part of the UTC timestamp at the time the system \
             timestamp was recorded."]
    TimestampMs(Field<profile::base::Uint16>),
    #[doc = "Millisecond part of the system timestamp"]
    SystemTimestampMs(Field<profile::base::Uint16>),
    Unknown {
        data:          Vec<u8>,
        field_def_num: u8,
    },
}
impl TimestampCorrelation {
    pub(crate) fn decode<T: ByteOrder>(
        buffer: &[u8],
        field_def_num: u8,
    ) -> error::Result<Self> {
        match field_def_num {
            253 => {
                Ok(TimestampCorrelation::Timestamp(Field {
                    raw_value:  profile::types::DateTime::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("s"),
                }))
            },
            0 => {
                Ok(TimestampCorrelation::FractionalTimestamp(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  Some(32768.0),
                    offset: None,
                    units:  Some("s"),
                }))
            },
            1 => {
                Ok(TimestampCorrelation::SystemTimestamp(Field {
                    raw_value:  profile::types::DateTime::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("s"),
                }))
            },
            2 => {
                Ok(TimestampCorrelation::FractionalSystemTimestamp(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  Some(32768.0),
                    offset: None,
                    units:  Some("s"),
                }))
            },
            3 => {
                Ok(TimestampCorrelation::LocalTimestamp(Field {
                    raw_value:  profile::types::LocalDateTime::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("s"),
                }))
            },
            4 => {
                Ok(TimestampCorrelation::TimestampMs(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("ms"),
                }))
            },
            5 => {
                Ok(TimestampCorrelation::SystemTimestampMs(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("ms"),
                }))
            },
            _ => {
                Ok(TimestampCorrelation::Unknown {
                    data: buffer.to_vec(),
                    field_def_num,
                })
            },
        }
    }
}
#[derive(Debug,Clone)]
pub enum Software {
    MessageIndex(Field<profile::types::MessageIndex>),
    Version(Field<profile::base::Uint16>),
    PartNumber(Field<profile::base::Utf8String>),
    Unknown { data:          Vec<u8>, field_def_num: u8 },
}
impl Software {
    pub(crate) fn decode<T: ByteOrder>(
        buffer: &[u8],
        field_def_num: u8,
    ) -> error::Result<Self> {
        match field_def_num {
            254 => {
                Ok(Software::MessageIndex(Field {
                    raw_value:  profile::types::MessageIndex::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            3 => {
                Ok(Software::Version(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  Some(100.0),
                    offset: None,
                    units:  None,
                }))
            },
            5 => {
                Ok(Software::PartNumber(Field {
                    raw_value:  profile::base::Utf8String::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            _ => {
                Ok(Software::Unknown {
                    data: buffer.to_vec(),
                    field_def_num,
                })
            },
        }
    }
}
#[derive(Debug,Clone)]
pub enum SlaveDevice {
    Manufacturer(Field<profile::types::Manufacturer>),
    Product(Field<profile::base::Uint16>),
    Unknown { data:          Vec<u8>, field_def_num: u8 },
}
impl SlaveDevice {
    pub(crate) fn decode<T: ByteOrder>(
        buffer: &[u8],
        field_def_num: u8,
    ) -> error::Result<Self> {
        match field_def_num {
            0 => {
                Ok(SlaveDevice::Manufacturer(Field {
                    raw_value:  profile::types::Manufacturer::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            1 => {
                Ok(SlaveDevice::Product(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            _ => {
                Ok(SlaveDevice::Unknown {
                    data: buffer.to_vec(),
                    field_def_num,
                })
            },
        }
    }
}
#[derive(Debug,Clone)]
pub enum Capabilities {
    #[doc = "Use language_bits_x types where x is index of array."]
    Languages(Field<profile::base::Uint8z>),
    #[doc = "Use sport_bits_x types where x is index of array."]
    Sports(Field<profile::types::SportBits0>),
    WorkoutsSupported(Field<profile::types::WorkoutCapabilities>),
    ConnectivitySupported(Field<profile::types::ConnectivityCapabilities>),
    Unknown {
        data:          Vec<u8>,
        field_def_num: u8,
    },
}
impl Capabilities {
    pub(crate) fn decode<T: ByteOrder>(
        buffer: &[u8],
        field_def_num: u8,
    ) -> error::Result<Self> {
        match field_def_num {
            0 => {
                Ok(Capabilities::Languages(Field {
                    raw_value:  profile::base::Uint8z::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            1 => {
                Ok(Capabilities::Sports(Field {
                    raw_value:  profile::types::SportBits0::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            21 => {
                Ok(Capabilities::WorkoutsSupported(Field {
                    raw_value:  profile::types::WorkoutCapabilities::decode::<T>(
                        buffer,
                    )?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            23 => {
                Ok(Capabilities::ConnectivitySupported(Field {
                    raw_value:  profile::types::ConnectivityCapabilities::decode::<
                        T,
                    >(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            _ => {
                Ok(Capabilities::Unknown {
                    data: buffer.to_vec(),
                    field_def_num,
                })
            },
        }
    }
}
#[derive(Debug,Clone)]
pub enum FileCapabilities {
    MessageIndex(Field<profile::types::MessageIndex>),
    Type(Field<profile::types::File>),
    Flags(Field<profile::types::FileFlags>),
    Directory(Field<profile::base::Utf8String>),
    MaxCount(Field<profile::base::Uint16>),
    MaxSize(Field<profile::base::Uint32>),
    Unknown { data:          Vec<u8>, field_def_num: u8 },
}
impl FileCapabilities {
    pub(crate) fn decode<T: ByteOrder>(
        buffer: &[u8],
        field_def_num: u8,
    ) -> error::Result<Self> {
        match field_def_num {
            254 => {
                Ok(FileCapabilities::MessageIndex(Field {
                    raw_value:  profile::types::MessageIndex::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            0 => {
                Ok(FileCapabilities::Type(Field {
                    raw_value:  profile::types::File::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            1 => {
                Ok(FileCapabilities::Flags(Field {
                    raw_value:  profile::types::FileFlags::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            2 => {
                Ok(FileCapabilities::Directory(Field {
                    raw_value:  profile::base::Utf8String::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            3 => {
                Ok(FileCapabilities::MaxCount(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            4 => {
                Ok(FileCapabilities::MaxSize(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("bytes"),
                }))
            },
            _ => {
                Ok(FileCapabilities::Unknown {
                    data: buffer.to_vec(),
                    field_def_num,
                })
            },
        }
    }
}
#[derive(Debug,Clone)]
pub enum MesgCapabilities {
    MessageIndex(Field<profile::types::MessageIndex>),
    File(Field<profile::types::File>),
    MesgNum(Field<profile::types::MesgNum>),
    CountType(Field<profile::types::MesgCount>),
    Count(Field<profile::base::Uint16>),
    Unknown { data:          Vec<u8>, field_def_num: u8 },
}
impl MesgCapabilities {
    pub(crate) fn decode<T: ByteOrder>(
        buffer: &[u8],
        field_def_num: u8,
    ) -> error::Result<Self> {
        match field_def_num {
            254 => {
                Ok(MesgCapabilities::MessageIndex(Field {
                    raw_value:  profile::types::MessageIndex::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            0 => {
                Ok(MesgCapabilities::File(Field {
                    raw_value:  profile::types::File::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            1 => {
                Ok(MesgCapabilities::MesgNum(Field {
                    raw_value:  profile::types::MesgNum::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            2 => {
                Ok(MesgCapabilities::CountType(Field {
                    raw_value:  profile::types::MesgCount::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            3 => {
                Ok(MesgCapabilities::Count(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            _ => {
                Ok(MesgCapabilities::Unknown {
                    data: buffer.to_vec(),
                    field_def_num,
                })
            },
        }
    }
}
#[derive(Debug,Clone)]
pub enum FieldCapabilities {
    MessageIndex(Field<profile::types::MessageIndex>),
    File(Field<profile::types::File>),
    MesgNum(Field<profile::types::MesgNum>),
    FieldNum(Field<profile::base::Uint8>),
    Count(Field<profile::base::Uint16>),
    Unknown { data:          Vec<u8>, field_def_num: u8 },
}
impl FieldCapabilities {
    pub(crate) fn decode<T: ByteOrder>(
        buffer: &[u8],
        field_def_num: u8,
    ) -> error::Result<Self> {
        match field_def_num {
            254 => {
                Ok(FieldCapabilities::MessageIndex(Field {
                    raw_value:  profile::types::MessageIndex::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            0 => {
                Ok(FieldCapabilities::File(Field {
                    raw_value:  profile::types::File::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            1 => {
                Ok(FieldCapabilities::MesgNum(Field {
                    raw_value:  profile::types::MesgNum::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            2 => {
                Ok(FieldCapabilities::FieldNum(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            3 => {
                Ok(FieldCapabilities::Count(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            _ => {
                Ok(FieldCapabilities::Unknown {
                    data: buffer.to_vec(),
                    field_def_num,
                })
            },
        }
    }
}
#[derive(Debug,Clone)]
pub enum DeviceSettings {
    #[doc = "Index into time zone arrays."]
    ActiveTimeZone(Field<profile::base::Uint8>),
    #[doc = "Offset from system time. Required to convert timestamp from \
             system time to UTC."]
    UtcOffset(Field<profile::base::Uint32>),
    #[doc = "Offset from system time."]
    TimeOffset(Field<profile::base::Uint32>),
    #[doc = "Display mode for the time"]
    TimeMode(Field<profile::types::TimeMode>),
    #[doc = "timezone offset in 1/4 hour increments"]
    TimeZoneOffset(Field<profile::base::Sint8>),
    #[doc = "Mode for backlight"]
    BacklightMode(Field<profile::types::BacklightMode>),
    #[doc = "Enabled state of the activity tracker functionality"]
    ActivityTrackerEnabled(Field<profile::base::Bool>),
    #[doc = "UTC timestamp used to set the devices clock and date"]
    ClockTime(Field<profile::types::DateTime>),
    #[doc = "Bitfield  to configure enabled screens for each supported loop"]
    PagesEnabled(Field<profile::base::Uint16>),
    #[doc = "Enabled state of the move alert"]
    MoveAlertEnabled(Field<profile::base::Bool>),
    #[doc = "Display mode for the date"]
    DateMode(Field<profile::types::DateMode>),
    DisplayOrientation(Field<profile::types::DisplayOrientation>),
    MountingSide(Field<profile::types::Side>),
    #[doc = "Bitfield to indicate one page as default for each supported loop"]
    DefaultPage(Field<profile::base::Uint16>),
    #[doc = "Minimum steps before an autosync can occur"]
    AutosyncMinSteps(Field<profile::base::Uint16>),
    #[doc = "Minimum minutes before an autosync can occur"]
    AutosyncMinTime(Field<profile::base::Uint16>),
    #[doc = "Enable auto-detect setting for the lactate threshold feature."]
    LactateThresholdAutodetectEnabled(Field<profile::base::Bool>),
    #[doc = "Automatically upload using BLE"]
    BleAutoUploadEnabled(Field<profile::base::Bool>),
    #[doc = "Helps to conserve battery by changing modes"]
    AutoSyncFrequency(Field<profile::types::AutoSyncFrequency>),
    #[doc = "Allows setting specific activities auto-activity detect \
             enabled/disabled settings"]
    AutoActivityDetect(Field<profile::types::AutoActivityDetect>),
    #[doc = "Number of screens configured to display"]
    NumberOfScreens(Field<profile::base::Uint8>),
    #[doc = "Smart Notification display orientation"]
    SmartNotificationDisplayOrientation(
        Field<profile::types::DisplayOrientation>,
    ),
    TapInterface(Field<profile::types::Switch>),
    Unknown {
        data:          Vec<u8>,
        field_def_num: u8,
    },
}
impl DeviceSettings {
    pub(crate) fn decode<T: ByteOrder>(
        buffer: &[u8],
        field_def_num: u8,
    ) -> error::Result<Self> {
        match field_def_num {
            0 => {
                Ok(DeviceSettings::ActiveTimeZone(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            1 => {
                Ok(DeviceSettings::UtcOffset(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            2 => {
                Ok(DeviceSettings::TimeOffset(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("s"),
                }))
            },
            4 => {
                Ok(DeviceSettings::TimeMode(Field {
                    raw_value:  profile::types::TimeMode::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            5 => {
                Ok(DeviceSettings::TimeZoneOffset(Field {
                    raw_value:  profile::base::Sint8::decode::<T>(buffer)?,
                    scale:  Some(4.0),
                    offset: None,
                    units:  Some("hr"),
                }))
            },
            12 => {
                Ok(DeviceSettings::BacklightMode(Field {
                    raw_value:  profile::types::BacklightMode::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            36 => {
                Ok(DeviceSettings::ActivityTrackerEnabled(Field {
                    raw_value:  profile::base::Bool::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            39 => {
                Ok(DeviceSettings::ClockTime(Field {
                    raw_value:  profile::types::DateTime::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            40 => {
                Ok(DeviceSettings::PagesEnabled(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            46 => {
                Ok(DeviceSettings::MoveAlertEnabled(Field {
                    raw_value:  profile::base::Bool::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            47 => {
                Ok(DeviceSettings::DateMode(Field {
                    raw_value:  profile::types::DateMode::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            55 => {
                Ok(DeviceSettings::DisplayOrientation(Field {
                    raw_value:  profile::types::DisplayOrientation::decode::<T>(
                        buffer,
                    )?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            56 => {
                Ok(DeviceSettings::MountingSide(Field {
                    raw_value:  profile::types::Side::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            57 => {
                Ok(DeviceSettings::DefaultPage(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            58 => {
                Ok(DeviceSettings::AutosyncMinSteps(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("steps"),
                }))
            },
            59 => {
                Ok(DeviceSettings::AutosyncMinTime(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("minutes"),
                }))
            },
            80 => {
                Ok(DeviceSettings::LactateThresholdAutodetectEnabled(Field {
                    raw_value:  profile::base::Bool::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            86 => {
                Ok(DeviceSettings::BleAutoUploadEnabled(Field {
                    raw_value:  profile::base::Bool::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            89 => {
                Ok(DeviceSettings::AutoSyncFrequency(Field {
                    raw_value:  profile::types::AutoSyncFrequency::decode::<T>(
                        buffer,
                    )?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            90 => {
                Ok(DeviceSettings::AutoActivityDetect(Field {
                    raw_value:  profile::types::AutoActivityDetect::decode::<T>(
                        buffer,
                    )?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            94 => {
                Ok(DeviceSettings::NumberOfScreens(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            95 => {
                Ok(DeviceSettings::SmartNotificationDisplayOrientation(Field {
                    raw_value:  profile::types::DisplayOrientation::decode::<T>(
                        buffer,
                    )?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            134 => {
                Ok(DeviceSettings::TapInterface(Field {
                    raw_value:  profile::types::Switch::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            _ => {
                Ok(DeviceSettings::Unknown {
                    data: buffer.to_vec(),
                    field_def_num,
                })
            },
        }
    }
}
#[derive(Debug,Clone)]
pub enum UserProfile {
    MessageIndex(Field<profile::types::MessageIndex>),
    FriendlyName(Field<profile::base::Utf8String>),
    Gender(Field<profile::types::Gender>),
    Age(Field<profile::base::Uint8>),
    Height(Field<profile::base::Uint8>),
    Weight(Field<profile::base::Uint16>),
    Language(Field<profile::types::Language>),
    ElevSetting(Field<profile::types::DisplayMeasure>),
    WeightSetting(Field<profile::types::DisplayMeasure>),
    RestingHeartRate(Field<profile::base::Uint8>),
    DefaultMaxRunningHeartRate(Field<profile::base::Uint8>),
    DefaultMaxBikingHeartRate(Field<profile::base::Uint8>),
    DefaultMaxHeartRate(Field<profile::base::Uint8>),
    HrSetting(Field<profile::types::DisplayHeart>),
    SpeedSetting(Field<profile::types::DisplayMeasure>),
    DistSetting(Field<profile::types::DisplayMeasure>),
    PowerSetting(Field<profile::types::DisplayPower>),
    ActivityClass(Field<profile::types::ActivityClass>),
    PositionSetting(Field<profile::types::DisplayPosition>),
    TemperatureSetting(Field<profile::types::DisplayMeasure>),
    LocalId(Field<profile::types::UserLocalId>),
    GlobalId(Field<profile::base::Bytes>),
    #[doc = "Typical wake time"]
    WakeTime(Field<profile::types::LocaltimeIntoDay>),
    #[doc = "Typical bed time"]
    SleepTime(Field<profile::types::LocaltimeIntoDay>),
    HeightSetting(Field<profile::types::DisplayMeasure>),
    #[doc = "User defined running step length set to 0 for auto length"]
    UserRunningStepLength(Field<profile::base::Uint16>),
    #[doc = "User defined walking step length set to 0 for auto length"]
    UserWalkingStepLength(Field<profile::base::Uint16>),
    DepthSetting(Field<profile::types::DisplayMeasure>),
    DiveCount(Field<profile::base::Uint32>),
    Unknown {
        data:          Vec<u8>,
        field_def_num: u8,
    },
}
impl UserProfile {
    pub(crate) fn decode<T: ByteOrder>(
        buffer: &[u8],
        field_def_num: u8,
    ) -> error::Result<Self> {
        match field_def_num {
            254 => {
                Ok(UserProfile::MessageIndex(Field {
                    raw_value:  profile::types::MessageIndex::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            0 => {
                Ok(UserProfile::FriendlyName(Field {
                    raw_value:  profile::base::Utf8String::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            1 => {
                Ok(UserProfile::Gender(Field {
                    raw_value:  profile::types::Gender::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            2 => {
                Ok(UserProfile::Age(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("years"),
                }))
            },
            3 => {
                Ok(UserProfile::Height(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  Some(100.0),
                    offset: None,
                    units:  Some("m"),
                }))
            },
            4 => {
                Ok(UserProfile::Weight(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  Some(10.0),
                    offset: None,
                    units:  Some("kg"),
                }))
            },
            5 => {
                Ok(UserProfile::Language(Field {
                    raw_value:  profile::types::Language::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            6 => {
                Ok(UserProfile::ElevSetting(Field {
                    raw_value:  profile::types::DisplayMeasure::decode::<T>(
                        buffer,
                    )?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            7 => {
                Ok(UserProfile::WeightSetting(Field {
                    raw_value:  profile::types::DisplayMeasure::decode::<T>(
                        buffer,
                    )?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            8 => {
                Ok(UserProfile::RestingHeartRate(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("bpm"),
                }))
            },
            9 => {
                Ok(UserProfile::DefaultMaxRunningHeartRate(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("bpm"),
                }))
            },
            10 => {
                Ok(UserProfile::DefaultMaxBikingHeartRate(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("bpm"),
                }))
            },
            11 => {
                Ok(UserProfile::DefaultMaxHeartRate(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("bpm"),
                }))
            },
            12 => {
                Ok(UserProfile::HrSetting(Field {
                    raw_value:  profile::types::DisplayHeart::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            13 => {
                Ok(UserProfile::SpeedSetting(Field {
                    raw_value:  profile::types::DisplayMeasure::decode::<T>(
                        buffer,
                    )?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            14 => {
                Ok(UserProfile::DistSetting(Field {
                    raw_value:  profile::types::DisplayMeasure::decode::<T>(
                        buffer,
                    )?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            16 => {
                Ok(UserProfile::PowerSetting(Field {
                    raw_value:  profile::types::DisplayPower::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            17 => {
                Ok(UserProfile::ActivityClass(Field {
                    raw_value:  profile::types::ActivityClass::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            18 => {
                Ok(UserProfile::PositionSetting(Field {
                    raw_value:  profile::types::DisplayPosition::decode::<T>(
                        buffer,
                    )?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            21 => {
                Ok(UserProfile::TemperatureSetting(Field {
                    raw_value:  profile::types::DisplayMeasure::decode::<T>(
                        buffer,
                    )?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            22 => {
                Ok(UserProfile::LocalId(Field {
                    raw_value:  profile::types::UserLocalId::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            23 => {
                Ok(UserProfile::GlobalId(Field {
                    raw_value:  profile::base::Bytes::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            28 => {
                Ok(UserProfile::WakeTime(Field {
                    raw_value:  profile::types::LocaltimeIntoDay::decode::<T>(
                        buffer,
                    )?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            29 => {
                Ok(UserProfile::SleepTime(Field {
                    raw_value:  profile::types::LocaltimeIntoDay::decode::<T>(
                        buffer,
                    )?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            30 => {
                Ok(UserProfile::HeightSetting(Field {
                    raw_value:  profile::types::DisplayMeasure::decode::<T>(
                        buffer,
                    )?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            31 => {
                Ok(UserProfile::UserRunningStepLength(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  Some(1000.0),
                    offset: None,
                    units:  Some("m"),
                }))
            },
            32 => {
                Ok(UserProfile::UserWalkingStepLength(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  Some(1000.0),
                    offset: None,
                    units:  Some("m"),
                }))
            },
            47 => {
                Ok(UserProfile::DepthSetting(Field {
                    raw_value:  profile::types::DisplayMeasure::decode::<T>(
                        buffer,
                    )?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            49 => {
                Ok(UserProfile::DiveCount(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            _ => {
                Ok(UserProfile::Unknown {
                    data: buffer.to_vec(),
                    field_def_num,
                })
            },
        }
    }
}
#[derive(Debug,Clone)]
pub enum HrmProfile {
    MessageIndex(Field<profile::types::MessageIndex>),
    Enabled(Field<profile::base::Bool>),
    HrmAntId(Field<profile::base::Uint16z>),
    LogHrv(Field<profile::base::Bool>),
    HrmAntIdTransType(Field<profile::base::Uint8z>),
    Unknown { data:          Vec<u8>, field_def_num: u8 },
}
impl HrmProfile {
    pub(crate) fn decode<T: ByteOrder>(
        buffer: &[u8],
        field_def_num: u8,
    ) -> error::Result<Self> {
        match field_def_num {
            254 => {
                Ok(HrmProfile::MessageIndex(Field {
                    raw_value:  profile::types::MessageIndex::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            0 => {
                Ok(HrmProfile::Enabled(Field {
                    raw_value:  profile::base::Bool::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            1 => {
                Ok(HrmProfile::HrmAntId(Field {
                    raw_value:  profile::base::Uint16z::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            2 => {
                Ok(HrmProfile::LogHrv(Field {
                    raw_value:  profile::base::Bool::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            3 => {
                Ok(HrmProfile::HrmAntIdTransType(Field {
                    raw_value:  profile::base::Uint8z::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            _ => {
                Ok(HrmProfile::Unknown {
                    data: buffer.to_vec(),
                    field_def_num,
                })
            },
        }
    }
}
#[derive(Debug,Clone)]
pub enum SdmProfile {
    MessageIndex(Field<profile::types::MessageIndex>),
    Enabled(Field<profile::base::Bool>),
    SdmAntId(Field<profile::base::Uint16z>),
    SdmCalFactor(Field<profile::base::Uint16>),
    Odometer(Field<profile::base::Uint32>),
    #[doc = "Use footpod for speed source instead of GPS"]
    SpeedSource(Field<profile::base::Bool>),
    SdmAntIdTransType(Field<profile::base::Uint8z>),
    #[doc = "Rollover counter that can be used to extend the odometer"]
    OdometerRollover(Field<profile::base::Uint8>),
    Unknown {
        data:          Vec<u8>,
        field_def_num: u8,
    },
}
impl SdmProfile {
    pub(crate) fn decode<T: ByteOrder>(
        buffer: &[u8],
        field_def_num: u8,
    ) -> error::Result<Self> {
        match field_def_num {
            254 => {
                Ok(SdmProfile::MessageIndex(Field {
                    raw_value:  profile::types::MessageIndex::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            0 => {
                Ok(SdmProfile::Enabled(Field {
                    raw_value:  profile::base::Bool::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            1 => {
                Ok(SdmProfile::SdmAntId(Field {
                    raw_value:  profile::base::Uint16z::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            2 => {
                Ok(SdmProfile::SdmCalFactor(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  Some(10.0),
                    offset: None,
                    units:  Some("%"),
                }))
            },
            3 => {
                Ok(SdmProfile::Odometer(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  Some(100.0),
                    offset: None,
                    units:  Some("m"),
                }))
            },
            4 => {
                Ok(SdmProfile::SpeedSource(Field {
                    raw_value:  profile::base::Bool::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            5 => {
                Ok(SdmProfile::SdmAntIdTransType(Field {
                    raw_value:  profile::base::Uint8z::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            7 => {
                Ok(SdmProfile::OdometerRollover(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            _ => {
                Ok(SdmProfile::Unknown {
                    data: buffer.to_vec(),
                    field_def_num,
                })
            },
        }
    }
}
#[derive(Debug,Clone)]
pub enum BikeProfile {
    MessageIndex(Field<profile::types::MessageIndex>),
    Name(Field<profile::base::Utf8String>),
    Sport(Field<profile::types::Sport>),
    SubSport(Field<profile::types::SubSport>),
    Odometer(Field<profile::base::Uint32>),
    BikeSpdAntId(Field<profile::base::Uint16z>),
    BikeCadAntId(Field<profile::base::Uint16z>),
    BikeSpdcadAntId(Field<profile::base::Uint16z>),
    BikePowerAntId(Field<profile::base::Uint16z>),
    CustomWheelsize(Field<profile::base::Uint16>),
    AutoWheelsize(Field<profile::base::Uint16>),
    BikeWeight(Field<profile::base::Uint16>),
    PowerCalFactor(Field<profile::base::Uint16>),
    AutoWheelCal(Field<profile::base::Bool>),
    AutoPowerZero(Field<profile::base::Bool>),
    Id(Field<profile::base::Uint8>),
    SpdEnabled(Field<profile::base::Bool>),
    CadEnabled(Field<profile::base::Bool>),
    SpdcadEnabled(Field<profile::base::Bool>),
    PowerEnabled(Field<profile::base::Bool>),
    CrankLength(Field<profile::base::Uint8>),
    Enabled(Field<profile::base::Bool>),
    BikeSpdAntIdTransType(Field<profile::base::Uint8z>),
    BikeCadAntIdTransType(Field<profile::base::Uint8z>),
    BikeSpdcadAntIdTransType(Field<profile::base::Uint8z>),
    BikePowerAntIdTransType(Field<profile::base::Uint8z>),
    #[doc = "Rollover counter that can be used to extend the odometer"]
    OdometerRollover(Field<profile::base::Uint8>),
    #[doc = "Number of front gears"]
    FrontGearNum(Field<profile::base::Uint8z>),
    #[doc = "Number of teeth on each gear 0 is innermost"]
    FrontGear(Field<profile::base::Uint8z>),
    #[doc = "Number of rear gears"]
    RearGearNum(Field<profile::base::Uint8z>),
    #[doc = "Number of teeth on each gear 0 is innermost"]
    RearGear(Field<profile::base::Uint8z>),
    ShimanoDi2Enabled(Field<profile::base::Bool>),
    Unknown {
        data:          Vec<u8>,
        field_def_num: u8,
    },
}
impl BikeProfile {
    pub(crate) fn decode<T: ByteOrder>(
        buffer: &[u8],
        field_def_num: u8,
    ) -> error::Result<Self> {
        match field_def_num {
            254 => {
                Ok(BikeProfile::MessageIndex(Field {
                    raw_value:  profile::types::MessageIndex::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            0 => {
                Ok(BikeProfile::Name(Field {
                    raw_value:  profile::base::Utf8String::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            1 => {
                Ok(BikeProfile::Sport(Field {
                    raw_value:  profile::types::Sport::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            2 => {
                Ok(BikeProfile::SubSport(Field {
                    raw_value:  profile::types::SubSport::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            3 => {
                Ok(BikeProfile::Odometer(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  Some(100.0),
                    offset: None,
                    units:  Some("m"),
                }))
            },
            4 => {
                Ok(BikeProfile::BikeSpdAntId(Field {
                    raw_value:  profile::base::Uint16z::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            5 => {
                Ok(BikeProfile::BikeCadAntId(Field {
                    raw_value:  profile::base::Uint16z::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            6 => {
                Ok(BikeProfile::BikeSpdcadAntId(Field {
                    raw_value:  profile::base::Uint16z::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            7 => {
                Ok(BikeProfile::BikePowerAntId(Field {
                    raw_value:  profile::base::Uint16z::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            8 => {
                Ok(BikeProfile::CustomWheelsize(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  Some(1000.0),
                    offset: None,
                    units:  Some("m"),
                }))
            },
            9 => {
                Ok(BikeProfile::AutoWheelsize(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  Some(1000.0),
                    offset: None,
                    units:  Some("m"),
                }))
            },
            10 => {
                Ok(BikeProfile::BikeWeight(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  Some(10.0),
                    offset: None,
                    units:  Some("kg"),
                }))
            },
            11 => {
                Ok(BikeProfile::PowerCalFactor(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  Some(10.0),
                    offset: None,
                    units:  Some("%"),
                }))
            },
            12 => {
                Ok(BikeProfile::AutoWheelCal(Field {
                    raw_value:  profile::base::Bool::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            13 => {
                Ok(BikeProfile::AutoPowerZero(Field {
                    raw_value:  profile::base::Bool::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            14 => {
                Ok(BikeProfile::Id(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            15 => {
                Ok(BikeProfile::SpdEnabled(Field {
                    raw_value:  profile::base::Bool::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            16 => {
                Ok(BikeProfile::CadEnabled(Field {
                    raw_value:  profile::base::Bool::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            17 => {
                Ok(BikeProfile::SpdcadEnabled(Field {
                    raw_value:  profile::base::Bool::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            18 => {
                Ok(BikeProfile::PowerEnabled(Field {
                    raw_value:  profile::base::Bool::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            19 => {
                Ok(BikeProfile::CrankLength(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  Some(2.0),
                    offset: None,
                    units:  Some("mm"),
                }))
            },
            20 => {
                Ok(BikeProfile::Enabled(Field {
                    raw_value:  profile::base::Bool::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            21 => {
                Ok(BikeProfile::BikeSpdAntIdTransType(Field {
                    raw_value:  profile::base::Uint8z::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            22 => {
                Ok(BikeProfile::BikeCadAntIdTransType(Field {
                    raw_value:  profile::base::Uint8z::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            23 => {
                Ok(BikeProfile::BikeSpdcadAntIdTransType(Field {
                    raw_value:  profile::base::Uint8z::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            24 => {
                Ok(BikeProfile::BikePowerAntIdTransType(Field {
                    raw_value:  profile::base::Uint8z::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            37 => {
                Ok(BikeProfile::OdometerRollover(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            38 => {
                Ok(BikeProfile::FrontGearNum(Field {
                    raw_value:  profile::base::Uint8z::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            39 => {
                Ok(BikeProfile::FrontGear(Field {
                    raw_value:  profile::base::Uint8z::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            40 => {
                Ok(BikeProfile::RearGearNum(Field {
                    raw_value:  profile::base::Uint8z::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            41 => {
                Ok(BikeProfile::RearGear(Field {
                    raw_value:  profile::base::Uint8z::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            44 => {
                Ok(BikeProfile::ShimanoDi2Enabled(Field {
                    raw_value:  profile::base::Bool::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            _ => {
                Ok(BikeProfile::Unknown {
                    data: buffer.to_vec(),
                    field_def_num,
                })
            },
        }
    }
}
#[derive(Debug,Clone)]
pub enum Connectivity {
    #[doc = "Use Bluetooth for connectivity features"]
    BluetoothEnabled(Field<profile::base::Bool>),
    #[doc = "Use Bluetooth Low Energy for connectivity features"]
    BluetoothLeEnabled(Field<profile::base::Bool>),
    #[doc = "Use ANT for connectivity features"]
    AntEnabled(Field<profile::base::Bool>),
    Name(Field<profile::base::Utf8String>),
    LiveTrackingEnabled(Field<profile::base::Bool>),
    WeatherConditionsEnabled(Field<profile::base::Bool>),
    WeatherAlertsEnabled(Field<profile::base::Bool>),
    AutoActivityUploadEnabled(Field<profile::base::Bool>),
    CourseDownloadEnabled(Field<profile::base::Bool>),
    WorkoutDownloadEnabled(Field<profile::base::Bool>),
    GpsEphemerisDownloadEnabled(Field<profile::base::Bool>),
    IncidentDetectionEnabled(Field<profile::base::Bool>),
    GrouptrackEnabled(Field<profile::base::Bool>),
    Unknown {
        data:          Vec<u8>,
        field_def_num: u8,
    },
}
impl Connectivity {
    pub(crate) fn decode<T: ByteOrder>(
        buffer: &[u8],
        field_def_num: u8,
    ) -> error::Result<Self> {
        match field_def_num {
            0 => {
                Ok(Connectivity::BluetoothEnabled(Field {
                    raw_value:  profile::base::Bool::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            1 => {
                Ok(Connectivity::BluetoothLeEnabled(Field {
                    raw_value:  profile::base::Bool::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            2 => {
                Ok(Connectivity::AntEnabled(Field {
                    raw_value:  profile::base::Bool::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            3 => {
                Ok(Connectivity::Name(Field {
                    raw_value:  profile::base::Utf8String::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            4 => {
                Ok(Connectivity::LiveTrackingEnabled(Field {
                    raw_value:  profile::base::Bool::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            5 => {
                Ok(Connectivity::WeatherConditionsEnabled(Field {
                    raw_value:  profile::base::Bool::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            6 => {
                Ok(Connectivity::WeatherAlertsEnabled(Field {
                    raw_value:  profile::base::Bool::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            7 => {
                Ok(Connectivity::AutoActivityUploadEnabled(Field {
                    raw_value:  profile::base::Bool::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            8 => {
                Ok(Connectivity::CourseDownloadEnabled(Field {
                    raw_value:  profile::base::Bool::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            9 => {
                Ok(Connectivity::WorkoutDownloadEnabled(Field {
                    raw_value:  profile::base::Bool::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            10 => {
                Ok(Connectivity::GpsEphemerisDownloadEnabled(Field {
                    raw_value:  profile::base::Bool::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            11 => {
                Ok(Connectivity::IncidentDetectionEnabled(Field {
                    raw_value:  profile::base::Bool::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            12 => {
                Ok(Connectivity::GrouptrackEnabled(Field {
                    raw_value:  profile::base::Bool::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            _ => {
                Ok(Connectivity::Unknown {
                    data: buffer.to_vec(),
                    field_def_num,
                })
            },
        }
    }
}
#[derive(Debug,Clone)]
pub enum WatchfaceSettings {
    MessageIndex(Field<profile::types::MessageIndex>),
    Mode(Field<profile::types::WatchfaceMode>),
    Layout(Field<profile::base::Bytes>),
    Unknown { data:          Vec<u8>, field_def_num: u8 },
}
impl WatchfaceSettings {
    pub(crate) fn decode<T: ByteOrder>(
        buffer: &[u8],
        field_def_num: u8,
    ) -> error::Result<Self> {
        match field_def_num {
            254 => {
                Ok(WatchfaceSettings::MessageIndex(Field {
                    raw_value:  profile::types::MessageIndex::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            0 => {
                Ok(WatchfaceSettings::Mode(Field {
                    raw_value:  profile::types::WatchfaceMode::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            1 => {
                Ok(WatchfaceSettings::Layout(Field {
                    raw_value:  profile::base::Bytes::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            _ => {
                Ok(WatchfaceSettings::Unknown {
                    data: buffer.to_vec(),
                    field_def_num,
                })
            },
        }
    }
}
#[derive(Debug,Clone)]
pub enum OhrSettings {
    Enabled(Field<profile::types::Switch>),
    Unknown { data:          Vec<u8>, field_def_num: u8 },
}
impl OhrSettings {
    pub(crate) fn decode<T: ByteOrder>(
        buffer: &[u8],
        field_def_num: u8,
    ) -> error::Result<Self> {
        match field_def_num {
            0 => {
                Ok(OhrSettings::Enabled(Field {
                    raw_value:  profile::types::Switch::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            _ => {
                Ok(OhrSettings::Unknown {
                    data: buffer.to_vec(),
                    field_def_num,
                })
            },
        }
    }
}
#[derive(Debug,Clone)]
pub enum ZonesTarget {
    MaxHeartRate(Field<profile::base::Uint8>),
    ThresholdHeartRate(Field<profile::base::Uint8>),
    FunctionalThresholdPower(Field<profile::base::Uint16>),
    HrCalcType(Field<profile::types::HrZoneCalc>),
    PwrCalcType(Field<profile::types::PwrZoneCalc>),
    Unknown { data:          Vec<u8>, field_def_num: u8 },
}
impl ZonesTarget {
    pub(crate) fn decode<T: ByteOrder>(
        buffer: &[u8],
        field_def_num: u8,
    ) -> error::Result<Self> {
        match field_def_num {
            1 => {
                Ok(ZonesTarget::MaxHeartRate(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            2 => {
                Ok(ZonesTarget::ThresholdHeartRate(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            3 => {
                Ok(ZonesTarget::FunctionalThresholdPower(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            5 => {
                Ok(ZonesTarget::HrCalcType(Field {
                    raw_value:  profile::types::HrZoneCalc::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            7 => {
                Ok(ZonesTarget::PwrCalcType(Field {
                    raw_value:  profile::types::PwrZoneCalc::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            _ => {
                Ok(ZonesTarget::Unknown {
                    data: buffer.to_vec(),
                    field_def_num,
                })
            },
        }
    }
}
#[derive(Debug,Clone)]
pub enum Sport {
    Sport(Field<profile::types::Sport>),
    SubSport(Field<profile::types::SubSport>),
    Name(Field<profile::base::Utf8String>),
    Unknown { data:          Vec<u8>, field_def_num: u8 },
}
impl Sport {
    pub(crate) fn decode<T: ByteOrder>(
        buffer: &[u8],
        field_def_num: u8,
    ) -> error::Result<Self> {
        match field_def_num {
            0 => {
                Ok(Sport::Sport(Field {
                    raw_value:  profile::types::Sport::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            1 => {
                Ok(Sport::SubSport(Field {
                    raw_value:  profile::types::SubSport::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            3 => {
                Ok(Sport::Name(Field {
                    raw_value:  profile::base::Utf8String::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            _ => {
                Ok(Sport::Unknown {
                    data: buffer.to_vec(),
                    field_def_num,
                })
            },
        }
    }
}
#[derive(Debug,Clone)]
pub enum HrZone {
    MessageIndex(Field<profile::types::MessageIndex>),
    HighBpm(Field<profile::base::Uint8>),
    Name(Field<profile::base::Utf8String>),
    Unknown { data:          Vec<u8>, field_def_num: u8 },
}
impl HrZone {
    pub(crate) fn decode<T: ByteOrder>(
        buffer: &[u8],
        field_def_num: u8,
    ) -> error::Result<Self> {
        match field_def_num {
            254 => {
                Ok(HrZone::MessageIndex(Field {
                    raw_value:  profile::types::MessageIndex::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            1 => {
                Ok(HrZone::HighBpm(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("bpm"),
                }))
            },
            2 => {
                Ok(HrZone::Name(Field {
                    raw_value:  profile::base::Utf8String::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            _ => {
                Ok(HrZone::Unknown {
                    data: buffer.to_vec(),
                    field_def_num,
                })
            },
        }
    }
}
#[derive(Debug,Clone)]
pub enum SpeedZone {
    MessageIndex(Field<profile::types::MessageIndex>),
    HighValue(Field<profile::base::Uint16>),
    Name(Field<profile::base::Utf8String>),
    Unknown { data:          Vec<u8>, field_def_num: u8 },
}
impl SpeedZone {
    pub(crate) fn decode<T: ByteOrder>(
        buffer: &[u8],
        field_def_num: u8,
    ) -> error::Result<Self> {
        match field_def_num {
            254 => {
                Ok(SpeedZone::MessageIndex(Field {
                    raw_value:  profile::types::MessageIndex::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            0 => {
                Ok(SpeedZone::HighValue(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  Some(1000.0),
                    offset: None,
                    units:  Some("m/s"),
                }))
            },
            1 => {
                Ok(SpeedZone::Name(Field {
                    raw_value:  profile::base::Utf8String::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            _ => {
                Ok(SpeedZone::Unknown {
                    data: buffer.to_vec(),
                    field_def_num,
                })
            },
        }
    }
}
#[derive(Debug,Clone)]
pub enum CadenceZone {
    MessageIndex(Field<profile::types::MessageIndex>),
    HighValue(Field<profile::base::Uint8>),
    Name(Field<profile::base::Utf8String>),
    Unknown { data:          Vec<u8>, field_def_num: u8 },
}
impl CadenceZone {
    pub(crate) fn decode<T: ByteOrder>(
        buffer: &[u8],
        field_def_num: u8,
    ) -> error::Result<Self> {
        match field_def_num {
            254 => {
                Ok(CadenceZone::MessageIndex(Field {
                    raw_value:  profile::types::MessageIndex::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            0 => {
                Ok(CadenceZone::HighValue(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("rpm"),
                }))
            },
            1 => {
                Ok(CadenceZone::Name(Field {
                    raw_value:  profile::base::Utf8String::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            _ => {
                Ok(CadenceZone::Unknown {
                    data: buffer.to_vec(),
                    field_def_num,
                })
            },
        }
    }
}
#[derive(Debug,Clone)]
pub enum PowerZone {
    MessageIndex(Field<profile::types::MessageIndex>),
    HighValue(Field<profile::base::Uint16>),
    Name(Field<profile::base::Utf8String>),
    Unknown { data:          Vec<u8>, field_def_num: u8 },
}
impl PowerZone {
    pub(crate) fn decode<T: ByteOrder>(
        buffer: &[u8],
        field_def_num: u8,
    ) -> error::Result<Self> {
        match field_def_num {
            254 => {
                Ok(PowerZone::MessageIndex(Field {
                    raw_value:  profile::types::MessageIndex::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            1 => {
                Ok(PowerZone::HighValue(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("watts"),
                }))
            },
            2 => {
                Ok(PowerZone::Name(Field {
                    raw_value:  profile::base::Utf8String::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            _ => {
                Ok(PowerZone::Unknown {
                    data: buffer.to_vec(),
                    field_def_num,
                })
            },
        }
    }
}
#[derive(Debug,Clone)]
pub enum MetZone {
    MessageIndex(Field<profile::types::MessageIndex>),
    HighBpm(Field<profile::base::Uint8>),
    Calories(Field<profile::base::Uint16>),
    FatCalories(Field<profile::base::Uint8>),
    Unknown { data:          Vec<u8>, field_def_num: u8 },
}
impl MetZone {
    pub(crate) fn decode<T: ByteOrder>(
        buffer: &[u8],
        field_def_num: u8,
    ) -> error::Result<Self> {
        match field_def_num {
            254 => {
                Ok(MetZone::MessageIndex(Field {
                    raw_value:  profile::types::MessageIndex::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            1 => {
                Ok(MetZone::HighBpm(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            2 => {
                Ok(MetZone::Calories(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  Some(10.0),
                    offset: None,
                    units:  Some("kcal / min"),
                }))
            },
            3 => {
                Ok(MetZone::FatCalories(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  Some(10.0),
                    offset: None,
                    units:  Some("kcal / min"),
                }))
            },
            _ => {
                Ok(MetZone::Unknown {
                    data: buffer.to_vec(),
                    field_def_num,
                })
            },
        }
    }
}
#[derive(Debug,Clone)]
pub enum DiveSettings {
    MessageIndex(Field<profile::types::MessageIndex>),
    Name(Field<profile::base::Utf8String>),
    Model(Field<profile::types::TissueModelType>),
    GfLow(Field<profile::base::Uint8>),
    GfHigh(Field<profile::base::Uint8>),
    WaterType(Field<profile::types::WaterType>),
    #[doc = "Fresh water is usually 1000; salt water is usually 1025"]
    WaterDensity(Field<profile::base::Float32>),
    #[doc = "Typically 1.40"]
    Po2Warn(Field<profile::base::Uint8>),
    #[doc = "Typically 1.60"]
    Po2Critical(Field<profile::base::Uint8>),
    Po2Deco(Field<profile::base::Uint8>),
    SafetyStopEnabled(Field<profile::base::Bool>),
    BottomDepth(Field<profile::base::Float32>),
    BottomTime(Field<profile::base::Uint32>),
    ApneaCountdownEnabled(Field<profile::base::Bool>),
    ApneaCountdownTime(Field<profile::base::Uint32>),
    BacklightMode(Field<profile::types::DiveBacklightMode>),
    BacklightBrightness(Field<profile::base::Uint8>),
    BacklightTimeout(Field<profile::types::BacklightTimeout>),
    #[doc = "Time between surfacing and ending the activity"]
    RepeatDiveInterval(Field<profile::base::Uint16>),
    #[doc = "Time at safety stop (if enabled)"]
    SafetyStopTime(Field<profile::base::Uint16>),
    HeartRateSourceType(Field<profile::types::SourceType>),
    HeartRateSource(Field<profile::base::Uint8>),
    Unknown {
        data:          Vec<u8>,
        field_def_num: u8,
    },
}
impl DiveSettings {
    pub(crate) fn decode<T: ByteOrder>(
        buffer: &[u8],
        field_def_num: u8,
    ) -> error::Result<Self> {
        match field_def_num {
            254 => {
                Ok(DiveSettings::MessageIndex(Field {
                    raw_value:  profile::types::MessageIndex::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            0 => {
                Ok(DiveSettings::Name(Field {
                    raw_value:  profile::base::Utf8String::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            1 => {
                Ok(DiveSettings::Model(Field {
                    raw_value:  profile::types::TissueModelType::decode::<T>(
                        buffer,
                    )?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            2 => {
                Ok(DiveSettings::GfLow(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("percent"),
                }))
            },
            3 => {
                Ok(DiveSettings::GfHigh(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("percent"),
                }))
            },
            4 => {
                Ok(DiveSettings::WaterType(Field {
                    raw_value:  profile::types::WaterType::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            5 => {
                Ok(DiveSettings::WaterDensity(Field {
                    raw_value:  profile::base::Float32::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("kg/m^3"),
                }))
            },
            6 => {
                Ok(DiveSettings::Po2Warn(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  Some(100.0),
                    offset: None,
                    units:  Some("percent"),
                }))
            },
            7 => {
                Ok(DiveSettings::Po2Critical(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  Some(100.0),
                    offset: None,
                    units:  Some("percent"),
                }))
            },
            8 => {
                Ok(DiveSettings::Po2Deco(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  Some(100.0),
                    offset: None,
                    units:  Some("percent"),
                }))
            },
            9 => {
                Ok(DiveSettings::SafetyStopEnabled(Field {
                    raw_value:  profile::base::Bool::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            10 => {
                Ok(DiveSettings::BottomDepth(Field {
                    raw_value:  profile::base::Float32::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            11 => {
                Ok(DiveSettings::BottomTime(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            12 => {
                Ok(DiveSettings::ApneaCountdownEnabled(Field {
                    raw_value:  profile::base::Bool::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            13 => {
                Ok(DiveSettings::ApneaCountdownTime(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            14 => {
                Ok(DiveSettings::BacklightMode(Field {
                    raw_value:  profile::types::DiveBacklightMode::decode::<T>(
                        buffer,
                    )?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            15 => {
                Ok(DiveSettings::BacklightBrightness(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            16 => {
                Ok(DiveSettings::BacklightTimeout(Field {
                    raw_value:  profile::types::BacklightTimeout::decode::<T>(
                        buffer,
                    )?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            17 => {
                Ok(DiveSettings::RepeatDiveInterval(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  Some(1.0),
                    offset: None,
                    units:  Some("s"),
                }))
            },
            18 => {
                Ok(DiveSettings::SafetyStopTime(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  Some(1.0),
                    offset: None,
                    units:  Some("s"),
                }))
            },
            19 => {
                Ok(DiveSettings::HeartRateSourceType(Field {
                    raw_value:  profile::types::SourceType::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            20 => {
                Ok(DiveSettings::HeartRateSource(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            _ => {
                Ok(DiveSettings::Unknown {
                    data: buffer.to_vec(),
                    field_def_num,
                })
            },
        }
    }
}
#[derive(Debug,Clone)]
pub enum DiveAlarm {
    #[doc = "Index of the alarm"]
    MessageIndex(Field<profile::types::MessageIndex>),
    Depth(Field<profile::base::Uint32>),
    Time(Field<profile::base::Sint32>),
    Enabled(Field<profile::base::Bool>),
    AlarmType(Field<profile::types::DiveAlarmType>),
    Sound(Field<profile::types::Tone>),
    DiveTypes(Field<profile::types::SubSport>),
    Unknown {
        data:          Vec<u8>,
        field_def_num: u8,
    },
}
impl DiveAlarm {
    pub(crate) fn decode<T: ByteOrder>(
        buffer: &[u8],
        field_def_num: u8,
    ) -> error::Result<Self> {
        match field_def_num {
            254 => {
                Ok(DiveAlarm::MessageIndex(Field {
                    raw_value:  profile::types::MessageIndex::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            0 => {
                Ok(DiveAlarm::Depth(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  Some(1000.0),
                    offset: None,
                    units:  Some("m"),
                }))
            },
            1 => {
                Ok(DiveAlarm::Time(Field {
                    raw_value:  profile::base::Sint32::decode::<T>(buffer)?,
                    scale:  Some(1.0),
                    offset: None,
                    units:  Some("s"),
                }))
            },
            2 => {
                Ok(DiveAlarm::Enabled(Field {
                    raw_value:  profile::base::Bool::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            3 => {
                Ok(DiveAlarm::AlarmType(Field {
                    raw_value:  profile::types::DiveAlarmType::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            4 => {
                Ok(DiveAlarm::Sound(Field {
                    raw_value:  profile::types::Tone::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            5 => {
                Ok(DiveAlarm::DiveTypes(Field {
                    raw_value:  profile::types::SubSport::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            _ => {
                Ok(DiveAlarm::Unknown {
                    data: buffer.to_vec(),
                    field_def_num,
                })
            },
        }
    }
}
#[derive(Debug,Clone)]
pub enum DiveGas {
    MessageIndex(Field<profile::types::MessageIndex>),
    HeliumContent(Field<profile::base::Uint8>),
    OxygenContent(Field<profile::base::Uint8>),
    Status(Field<profile::types::DiveGasStatus>),
    Unknown { data:          Vec<u8>, field_def_num: u8 },
}
impl DiveGas {
    pub(crate) fn decode<T: ByteOrder>(
        buffer: &[u8],
        field_def_num: u8,
    ) -> error::Result<Self> {
        match field_def_num {
            254 => {
                Ok(DiveGas::MessageIndex(Field {
                    raw_value:  profile::types::MessageIndex::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            0 => {
                Ok(DiveGas::HeliumContent(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("percent"),
                }))
            },
            1 => {
                Ok(DiveGas::OxygenContent(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("percent"),
                }))
            },
            2 => {
                Ok(DiveGas::Status(Field {
                    raw_value:  profile::types::DiveGasStatus::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            _ => {
                Ok(DiveGas::Unknown {
                    data: buffer.to_vec(),
                    field_def_num,
                })
            },
        }
    }
}
#[derive(Debug,Clone)]
pub enum Goal {
    MessageIndex(Field<profile::types::MessageIndex>),
    Sport(Field<profile::types::Sport>),
    SubSport(Field<profile::types::SubSport>),
    StartDate(Field<profile::types::DateTime>),
    EndDate(Field<profile::types::DateTime>),
    Type(Field<profile::types::Goal>),
    Value(Field<profile::base::Uint32>),
    Repeat(Field<profile::base::Bool>),
    TargetValue(Field<profile::base::Uint32>),
    Recurrence(Field<profile::types::GoalRecurrence>),
    RecurrenceValue(Field<profile::base::Uint16>),
    Enabled(Field<profile::base::Bool>),
    Source(Field<profile::types::GoalSource>),
    Unknown { data:          Vec<u8>, field_def_num: u8 },
}
impl Goal {
    pub(crate) fn decode<T: ByteOrder>(
        buffer: &[u8],
        field_def_num: u8,
    ) -> error::Result<Self> {
        match field_def_num {
            254 => {
                Ok(Goal::MessageIndex(Field {
                    raw_value:  profile::types::MessageIndex::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            0 => {
                Ok(Goal::Sport(Field {
                    raw_value:  profile::types::Sport::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            1 => {
                Ok(Goal::SubSport(Field {
                    raw_value:  profile::types::SubSport::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            2 => {
                Ok(Goal::StartDate(Field {
                    raw_value:  profile::types::DateTime::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            3 => {
                Ok(Goal::EndDate(Field {
                    raw_value:  profile::types::DateTime::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            4 => {
                Ok(Goal::Type(Field {
                    raw_value:  profile::types::Goal::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            5 => {
                Ok(Goal::Value(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            6 => {
                Ok(Goal::Repeat(Field {
                    raw_value:  profile::base::Bool::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            7 => {
                Ok(Goal::TargetValue(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            8 => {
                Ok(Goal::Recurrence(Field {
                    raw_value:  profile::types::GoalRecurrence::decode::<T>(
                        buffer,
                    )?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            9 => {
                Ok(Goal::RecurrenceValue(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            10 => {
                Ok(Goal::Enabled(Field {
                    raw_value:  profile::base::Bool::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            11 => {
                Ok(Goal::Source(Field {
                    raw_value:  profile::types::GoalSource::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            _ => {
                Ok(Goal::Unknown {
                    data: buffer.to_vec(),
                    field_def_num,
                })
            },
        }
    }
}
#[derive(Debug,Clone)]
pub enum Activity {
    Timestamp(Field<profile::types::DateTime>),
    #[doc = "Exclude pauses"]
    TotalTimerTime(Field<profile::base::Uint32>),
    NumSessions(Field<profile::base::Uint16>),
    Type(Field<profile::types::Activity>),
    Event(Field<profile::types::Event>),
    EventType(Field<profile::types::EventType>),
    #[doc = "timestamp epoch expressed in local time, used to convert \
             activity timestamps to local time "]
    LocalTimestamp(Field<profile::types::LocalDateTime>),
    EventGroup(Field<profile::base::Uint8>),
    Unknown {
        data:          Vec<u8>,
        field_def_num: u8,
    },
}
impl Activity {
    pub(crate) fn decode<T: ByteOrder>(
        buffer: &[u8],
        field_def_num: u8,
    ) -> error::Result<Self> {
        match field_def_num {
            253 => {
                Ok(Activity::Timestamp(Field {
                    raw_value:  profile::types::DateTime::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            0 => {
                Ok(Activity::TotalTimerTime(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  Some(1000.0),
                    offset: None,
                    units:  Some("s"),
                }))
            },
            1 => {
                Ok(Activity::NumSessions(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            2 => {
                Ok(Activity::Type(Field {
                    raw_value:  profile::types::Activity::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            3 => {
                Ok(Activity::Event(Field {
                    raw_value:  profile::types::Event::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            4 => {
                Ok(Activity::EventType(Field {
                    raw_value:  profile::types::EventType::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            5 => {
                Ok(Activity::LocalTimestamp(Field {
                    raw_value:  profile::types::LocalDateTime::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            6 => {
                Ok(Activity::EventGroup(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            _ => {
                Ok(Activity::Unknown {
                    data: buffer.to_vec(),
                    field_def_num,
                })
            },
        }
    }
}
#[derive(Debug,Clone)]
pub enum Session {
    #[doc = "Selected bit is set for the current session."]
    MessageIndex(Field<profile::types::MessageIndex>),
    #[doc = "Sesson end time."]
    Timestamp(Field<profile::types::DateTime>),
    #[doc = "session"]
    Event(Field<profile::types::Event>),
    #[doc = "stop"]
    EventType(Field<profile::types::EventType>),
    StartTime(Field<profile::types::DateTime>),
    StartPositionLat(Field<profile::base::Sint32>),
    StartPositionLong(Field<profile::base::Sint32>),
    Sport(Field<profile::types::Sport>),
    SubSport(Field<profile::types::SubSport>),
    #[doc = "Time (includes pauses)"]
    TotalElapsedTime(Field<profile::base::Uint32>),
    #[doc = "Timer Time (excludes pauses)"]
    TotalTimerTime(Field<profile::base::Uint32>),
    TotalDistance(Field<profile::base::Uint32>),
    TotalCycles(Field<profile::base::Uint32>),
    TotalCalories(Field<profile::base::Uint16>),
    TotalFatCalories(Field<profile::base::Uint16>),
    #[doc = "total_distance / total_timer_time"]
    AvgSpeed(Field<profile::base::Uint16>),
    MaxSpeed(Field<profile::base::Uint16>),
    #[doc = "average heart rate (excludes pause time)"]
    AvgHeartRate(Field<profile::base::Uint8>),
    MaxHeartRate(Field<profile::base::Uint8>),
    #[doc = "total_cycles / total_timer_time if non_zero_avg_cadence \
             otherwise total_cycles / total_elapsed_time"]
    AvgCadence(Field<profile::base::Uint8>),
    MaxCadence(Field<profile::base::Uint8>),
    #[doc = "total_power / total_timer_time if non_zero_avg_power otherwise \
             total_power / total_elapsed_time"]
    AvgPower(Field<profile::base::Uint16>),
    MaxPower(Field<profile::base::Uint16>),
    TotalAscent(Field<profile::base::Uint16>),
    TotalDescent(Field<profile::base::Uint16>),
    TotalTrainingEffect(Field<profile::base::Uint8>),
    FirstLapIndex(Field<profile::base::Uint16>),
    NumLaps(Field<profile::base::Uint16>),
    EventGroup(Field<profile::base::Uint8>),
    Trigger(Field<profile::types::SessionTrigger>),
    NecLat(Field<profile::base::Sint32>),
    NecLong(Field<profile::base::Sint32>),
    SwcLat(Field<profile::base::Sint32>),
    SwcLong(Field<profile::base::Sint32>),
    NormalizedPower(Field<profile::base::Uint16>),
    TrainingStressScore(Field<profile::base::Uint16>),
    IntensityFactor(Field<profile::base::Uint16>),
    LeftRightBalance(Field<profile::types::LeftRightBalance100>),
    AvgStrokeCount(Field<profile::base::Uint32>),
    AvgStrokeDistance(Field<profile::base::Uint16>),
    SwimStroke(Field<profile::types::SwimStroke>),
    PoolLength(Field<profile::base::Uint16>),
    ThresholdPower(Field<profile::base::Uint16>),
    PoolLengthUnit(Field<profile::types::DisplayMeasure>),
    #[doc = "# of active lengths of swim pool"]
    NumActiveLengths(Field<profile::base::Uint16>),
    TotalWork(Field<profile::base::Uint32>),
    AvgAltitude(Field<profile::base::Uint16>),
    MaxAltitude(Field<profile::base::Uint16>),
    GpsAccuracy(Field<profile::base::Uint8>),
    AvgGrade(Field<profile::base::Sint16>),
    AvgPosGrade(Field<profile::base::Sint16>),
    AvgNegGrade(Field<profile::base::Sint16>),
    MaxPosGrade(Field<profile::base::Sint16>),
    MaxNegGrade(Field<profile::base::Sint16>),
    AvgTemperature(Field<profile::base::Sint8>),
    MaxTemperature(Field<profile::base::Sint8>),
    TotalMovingTime(Field<profile::base::Uint32>),
    AvgPosVerticalSpeed(Field<profile::base::Sint16>),
    AvgNegVerticalSpeed(Field<profile::base::Sint16>),
    MaxPosVerticalSpeed(Field<profile::base::Sint16>),
    MaxNegVerticalSpeed(Field<profile::base::Sint16>),
    MinHeartRate(Field<profile::base::Uint8>),
    TimeInHrZone(Field<profile::base::Uint32>),
    TimeInSpeedZone(Field<profile::base::Uint32>),
    TimeInCadenceZone(Field<profile::base::Uint32>),
    TimeInPowerZone(Field<profile::base::Uint32>),
    AvgLapTime(Field<profile::base::Uint32>),
    BestLapIndex(Field<profile::base::Uint16>),
    MinAltitude(Field<profile::base::Uint16>),
    PlayerScore(Field<profile::base::Uint16>),
    OpponentScore(Field<profile::base::Uint16>),
    OpponentName(Field<profile::base::Utf8String>),
    #[doc = "stroke_type enum used as the index"]
    StrokeCount(Field<profile::base::Uint16>),
    #[doc = "zone number used as the index"]
    ZoneCount(Field<profile::base::Uint16>),
    MaxBallSpeed(Field<profile::base::Uint16>),
    AvgBallSpeed(Field<profile::base::Uint16>),
    AvgVerticalOscillation(Field<profile::base::Uint16>),
    AvgStanceTimePercent(Field<profile::base::Uint16>),
    AvgStanceTime(Field<profile::base::Uint16>),
    #[doc = "fractional part of the avg_cadence"]
    AvgFractionalCadence(Field<profile::base::Uint8>),
    #[doc = "fractional part of the max_cadence"]
    MaxFractionalCadence(Field<profile::base::Uint8>),
    #[doc = "fractional part of the total_cycles"]
    TotalFractionalCycles(Field<profile::base::Uint8>),
    #[doc = "Avg saturated and unsaturated hemoglobin"]
    AvgTotalHemoglobinConc(Field<profile::base::Uint16>),
    #[doc = "Min saturated and unsaturated hemoglobin"]
    MinTotalHemoglobinConc(Field<profile::base::Uint16>),
    #[doc = "Max saturated and unsaturated hemoglobin"]
    MaxTotalHemoglobinConc(Field<profile::base::Uint16>),
    #[doc = "Avg percentage of hemoglobin saturated with oxygen"]
    AvgSaturatedHemoglobinPercent(Field<profile::base::Uint16>),
    #[doc = "Min percentage of hemoglobin saturated with oxygen"]
    MinSaturatedHemoglobinPercent(Field<profile::base::Uint16>),
    #[doc = "Max percentage of hemoglobin saturated with oxygen"]
    MaxSaturatedHemoglobinPercent(Field<profile::base::Uint16>),
    AvgLeftTorqueEffectiveness(Field<profile::base::Uint8>),
    AvgRightTorqueEffectiveness(Field<profile::base::Uint8>),
    AvgLeftPedalSmoothness(Field<profile::base::Uint8>),
    AvgRightPedalSmoothness(Field<profile::base::Uint8>),
    AvgCombinedPedalSmoothness(Field<profile::base::Uint8>),
    SportIndex(Field<profile::base::Uint8>),
    #[doc = "Total time spend in the standing position"]
    TimeStanding(Field<profile::base::Uint32>),
    #[doc = "Number of transitions to the standing state"]
    StandCount(Field<profile::base::Uint16>),
    #[doc = "Average platform center offset Left"]
    AvgLeftPco(Field<profile::base::Sint8>),
    #[doc = "Average platform center offset Right"]
    AvgRightPco(Field<profile::base::Sint8>),
    #[doc = "Average left power phase angles. Indexes defined by \
             power_phase_type."]
    AvgLeftPowerPhase(Field<profile::base::Uint8>),
    #[doc = "Average left power phase peak angles. Data raw_value indexes defined \
             by power_phase_type."]
    AvgLeftPowerPhasePeak(Field<profile::base::Uint8>),
    #[doc = "Average right power phase angles. Data raw_value indexes defined by \
             power_phase_type."]
    AvgRightPowerPhase(Field<profile::base::Uint8>),
    #[doc = "Average right power phase peak angles data raw_value indexes  \
             defined by power_phase_type."]
    AvgRightPowerPhasePeak(Field<profile::base::Uint8>),
    #[doc = "Average power by position. Data raw_value indexes defined by \
             rider_position_type."]
    AvgPowerPosition(Field<profile::base::Uint16>),
    #[doc = "Maximum power by position. Data raw_value indexes defined by \
             rider_position_type."]
    MaxPowerPosition(Field<profile::base::Uint16>),
    #[doc = "Average cadence by position. Data raw_value indexes defined by \
             rider_position_type."]
    AvgCadencePosition(Field<profile::base::Uint8>),
    #[doc = "Maximum cadence by position. Data raw_value indexes defined by \
             rider_position_type."]
    MaxCadencePosition(Field<profile::base::Uint8>),
    #[doc = "total_distance / total_timer_time"]
    EnhancedAvgSpeed(Field<profile::base::Uint32>),
    EnhancedMaxSpeed(Field<profile::base::Uint32>),
    EnhancedAvgAltitude(Field<profile::base::Uint32>),
    EnhancedMinAltitude(Field<profile::base::Uint32>),
    EnhancedMaxAltitude(Field<profile::base::Uint32>),
    #[doc = "lev average motor power during session"]
    AvgLevMotorPower(Field<profile::base::Uint16>),
    #[doc = "lev maximum motor power during session"]
    MaxLevMotorPower(Field<profile::base::Uint16>),
    #[doc = "lev battery consumption during session"]
    LevBatteryConsumption(Field<profile::base::Uint8>),
    AvgVerticalRatio(Field<profile::base::Uint16>),
    AvgStanceTimeBalance(Field<profile::base::Uint16>),
    AvgStepLength(Field<profile::base::Uint16>),
    TotalAnaerobicTrainingEffect(Field<profile::base::Uint8>),
    AvgVam(Field<profile::base::Uint16>),
    Unknown {
        data:          Vec<u8>,
        field_def_num: u8,
    },
}
impl Session {
    pub(crate) fn decode<T: ByteOrder>(
        buffer: &[u8],
        field_def_num: u8,
    ) -> error::Result<Self> {
        match field_def_num {
            254 => {
                Ok(Session::MessageIndex(Field {
                    raw_value:  profile::types::MessageIndex::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            253 => {
                Ok(Session::Timestamp(Field {
                    raw_value:  profile::types::DateTime::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("s"),
                }))
            },
            0 => {
                Ok(Session::Event(Field {
                    raw_value:  profile::types::Event::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            1 => {
                Ok(Session::EventType(Field {
                    raw_value:  profile::types::EventType::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            2 => {
                Ok(Session::StartTime(Field {
                    raw_value:  profile::types::DateTime::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            3 => {
                Ok(Session::StartPositionLat(Field {
                    raw_value:  profile::base::Sint32::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("semicircles"),
                }))
            },
            4 => {
                Ok(Session::StartPositionLong(Field {
                    raw_value:  profile::base::Sint32::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("semicircles"),
                }))
            },
            5 => {
                Ok(Session::Sport(Field {
                    raw_value:  profile::types::Sport::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            6 => {
                Ok(Session::SubSport(Field {
                    raw_value:  profile::types::SubSport::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            7 => {
                Ok(Session::TotalElapsedTime(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  Some(1000.0),
                    offset: None,
                    units:  Some("s"),
                }))
            },
            8 => {
                Ok(Session::TotalTimerTime(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  Some(1000.0),
                    offset: None,
                    units:  Some("s"),
                }))
            },
            9 => {
                Ok(Session::TotalDistance(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  Some(100.0),
                    offset: None,
                    units:  Some("m"),
                }))
            },
            10 => {
                Ok(Session::TotalCycles(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("cycles"),
                }))
            },
            11 => {
                Ok(Session::TotalCalories(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("kcal"),
                }))
            },
            13 => {
                Ok(Session::TotalFatCalories(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("kcal"),
                }))
            },
            14 => {
                Ok(Session::AvgSpeed(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  Some(1000.0),
                    offset: None,
                    units:  Some("m/s"),
                }))
            },
            15 => {
                Ok(Session::MaxSpeed(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  Some(1000.0),
                    offset: None,
                    units:  Some("m/s"),
                }))
            },
            16 => {
                Ok(Session::AvgHeartRate(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("bpm"),
                }))
            },
            17 => {
                Ok(Session::MaxHeartRate(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("bpm"),
                }))
            },
            18 => {
                Ok(Session::AvgCadence(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("rpm"),
                }))
            },
            19 => {
                Ok(Session::MaxCadence(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("rpm"),
                }))
            },
            20 => {
                Ok(Session::AvgPower(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("watts"),
                }))
            },
            21 => {
                Ok(Session::MaxPower(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("watts"),
                }))
            },
            22 => {
                Ok(Session::TotalAscent(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("m"),
                }))
            },
            23 => {
                Ok(Session::TotalDescent(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("m"),
                }))
            },
            24 => {
                Ok(Session::TotalTrainingEffect(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  Some(10.0),
                    offset: None,
                    units:  None,
                }))
            },
            25 => {
                Ok(Session::FirstLapIndex(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            26 => {
                Ok(Session::NumLaps(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            27 => {
                Ok(Session::EventGroup(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            28 => {
                Ok(Session::Trigger(Field {
                    raw_value:  profile::types::SessionTrigger::decode::<T>(
                        buffer,
                    )?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            29 => {
                Ok(Session::NecLat(Field {
                    raw_value:  profile::base::Sint32::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("semicircles"),
                }))
            },
            30 => {
                Ok(Session::NecLong(Field {
                    raw_value:  profile::base::Sint32::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("semicircles"),
                }))
            },
            31 => {
                Ok(Session::SwcLat(Field {
                    raw_value:  profile::base::Sint32::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("semicircles"),
                }))
            },
            32 => {
                Ok(Session::SwcLong(Field {
                    raw_value:  profile::base::Sint32::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("semicircles"),
                }))
            },
            34 => {
                Ok(Session::NormalizedPower(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("watts"),
                }))
            },
            35 => {
                Ok(Session::TrainingStressScore(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  Some(10.0),
                    offset: None,
                    units:  Some("tss"),
                }))
            },
            36 => {
                Ok(Session::IntensityFactor(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  Some(1000.0),
                    offset: None,
                    units:  Some("if"),
                }))
            },
            37 => {
                Ok(Session::LeftRightBalance(Field {
                    raw_value:  profile::types::LeftRightBalance100::decode::<T>(
                        buffer,
                    )?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            41 => {
                Ok(Session::AvgStrokeCount(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  Some(10.0),
                    offset: None,
                    units:  Some("strokes/lap"),
                }))
            },
            42 => {
                Ok(Session::AvgStrokeDistance(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  Some(100.0),
                    offset: None,
                    units:  Some("m"),
                }))
            },
            43 => {
                Ok(Session::SwimStroke(Field {
                    raw_value:  profile::types::SwimStroke::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("swim_stroke"),
                }))
            },
            44 => {
                Ok(Session::PoolLength(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  Some(100.0),
                    offset: None,
                    units:  Some("m"),
                }))
            },
            45 => {
                Ok(Session::ThresholdPower(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("watts"),
                }))
            },
            46 => {
                Ok(Session::PoolLengthUnit(Field {
                    raw_value:  profile::types::DisplayMeasure::decode::<T>(
                        buffer,
                    )?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            47 => {
                Ok(Session::NumActiveLengths(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("lengths"),
                }))
            },
            48 => {
                Ok(Session::TotalWork(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("J"),
                }))
            },
            49 => {
                Ok(Session::AvgAltitude(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  Some(5.0),
                    offset: Some(500.0),
                    units:  Some("m"),
                }))
            },
            50 => {
                Ok(Session::MaxAltitude(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  Some(5.0),
                    offset: Some(500.0),
                    units:  Some("m"),
                }))
            },
            51 => {
                Ok(Session::GpsAccuracy(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("m"),
                }))
            },
            52 => {
                Ok(Session::AvgGrade(Field {
                    raw_value:  profile::base::Sint16::decode::<T>(buffer)?,
                    scale:  Some(100.0),
                    offset: None,
                    units:  Some("%"),
                }))
            },
            53 => {
                Ok(Session::AvgPosGrade(Field {
                    raw_value:  profile::base::Sint16::decode::<T>(buffer)?,
                    scale:  Some(100.0),
                    offset: None,
                    units:  Some("%"),
                }))
            },
            54 => {
                Ok(Session::AvgNegGrade(Field {
                    raw_value:  profile::base::Sint16::decode::<T>(buffer)?,
                    scale:  Some(100.0),
                    offset: None,
                    units:  Some("%"),
                }))
            },
            55 => {
                Ok(Session::MaxPosGrade(Field {
                    raw_value:  profile::base::Sint16::decode::<T>(buffer)?,
                    scale:  Some(100.0),
                    offset: None,
                    units:  Some("%"),
                }))
            },
            56 => {
                Ok(Session::MaxNegGrade(Field {
                    raw_value:  profile::base::Sint16::decode::<T>(buffer)?,
                    scale:  Some(100.0),
                    offset: None,
                    units:  Some("%"),
                }))
            },
            57 => {
                Ok(Session::AvgTemperature(Field {
                    raw_value:  profile::base::Sint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("C"),
                }))
            },
            58 => {
                Ok(Session::MaxTemperature(Field {
                    raw_value:  profile::base::Sint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("C"),
                }))
            },
            59 => {
                Ok(Session::TotalMovingTime(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  Some(1000.0),
                    offset: None,
                    units:  Some("s"),
                }))
            },
            60 => {
                Ok(Session::AvgPosVerticalSpeed(Field {
                    raw_value:  profile::base::Sint16::decode::<T>(buffer)?,
                    scale:  Some(1000.0),
                    offset: None,
                    units:  Some("m/s"),
                }))
            },
            61 => {
                Ok(Session::AvgNegVerticalSpeed(Field {
                    raw_value:  profile::base::Sint16::decode::<T>(buffer)?,
                    scale:  Some(1000.0),
                    offset: None,
                    units:  Some("m/s"),
                }))
            },
            62 => {
                Ok(Session::MaxPosVerticalSpeed(Field {
                    raw_value:  profile::base::Sint16::decode::<T>(buffer)?,
                    scale:  Some(1000.0),
                    offset: None,
                    units:  Some("m/s"),
                }))
            },
            63 => {
                Ok(Session::MaxNegVerticalSpeed(Field {
                    raw_value:  profile::base::Sint16::decode::<T>(buffer)?,
                    scale:  Some(1000.0),
                    offset: None,
                    units:  Some("m/s"),
                }))
            },
            64 => {
                Ok(Session::MinHeartRate(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("bpm"),
                }))
            },
            65 => {
                Ok(Session::TimeInHrZone(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  Some(1000.0),
                    offset: None,
                    units:  Some("s"),
                }))
            },
            66 => {
                Ok(Session::TimeInSpeedZone(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  Some(1000.0),
                    offset: None,
                    units:  Some("s"),
                }))
            },
            67 => {
                Ok(Session::TimeInCadenceZone(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  Some(1000.0),
                    offset: None,
                    units:  Some("s"),
                }))
            },
            68 => {
                Ok(Session::TimeInPowerZone(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  Some(1000.0),
                    offset: None,
                    units:  Some("s"),
                }))
            },
            69 => {
                Ok(Session::AvgLapTime(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  Some(1000.0),
                    offset: None,
                    units:  Some("s"),
                }))
            },
            70 => {
                Ok(Session::BestLapIndex(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            71 => {
                Ok(Session::MinAltitude(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  Some(5.0),
                    offset: Some(500.0),
                    units:  Some("m"),
                }))
            },
            82 => {
                Ok(Session::PlayerScore(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            83 => {
                Ok(Session::OpponentScore(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            84 => {
                Ok(Session::OpponentName(Field {
                    raw_value:  profile::base::Utf8String::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            85 => {
                Ok(Session::StrokeCount(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("counts"),
                }))
            },
            86 => {
                Ok(Session::ZoneCount(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("counts"),
                }))
            },
            87 => {
                Ok(Session::MaxBallSpeed(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  Some(100.0),
                    offset: None,
                    units:  Some("m/s"),
                }))
            },
            88 => {
                Ok(Session::AvgBallSpeed(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  Some(100.0),
                    offset: None,
                    units:  Some("m/s"),
                }))
            },
            89 => {
                Ok(Session::AvgVerticalOscillation(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  Some(10.0),
                    offset: None,
                    units:  Some("mm"),
                }))
            },
            90 => {
                Ok(Session::AvgStanceTimePercent(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  Some(100.0),
                    offset: None,
                    units:  Some("percent"),
                }))
            },
            91 => {
                Ok(Session::AvgStanceTime(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  Some(10.0),
                    offset: None,
                    units:  Some("ms"),
                }))
            },
            92 => {
                Ok(Session::AvgFractionalCadence(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  Some(128.0),
                    offset: None,
                    units:  Some("rpm"),
                }))
            },
            93 => {
                Ok(Session::MaxFractionalCadence(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  Some(128.0),
                    offset: None,
                    units:  Some("rpm"),
                }))
            },
            94 => {
                Ok(Session::TotalFractionalCycles(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  Some(128.0),
                    offset: None,
                    units:  Some("cycles"),
                }))
            },
            95 => {
                Ok(Session::AvgTotalHemoglobinConc(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  Some(100.0),
                    offset: None,
                    units:  Some("g/dL"),
                }))
            },
            96 => {
                Ok(Session::MinTotalHemoglobinConc(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  Some(100.0),
                    offset: None,
                    units:  Some("g/dL"),
                }))
            },
            97 => {
                Ok(Session::MaxTotalHemoglobinConc(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  Some(100.0),
                    offset: None,
                    units:  Some("g/dL"),
                }))
            },
            98 => {
                Ok(Session::AvgSaturatedHemoglobinPercent(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  Some(10.0),
                    offset: None,
                    units:  Some("%"),
                }))
            },
            99 => {
                Ok(Session::MinSaturatedHemoglobinPercent(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  Some(10.0),
                    offset: None,
                    units:  Some("%"),
                }))
            },
            100 => {
                Ok(Session::MaxSaturatedHemoglobinPercent(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  Some(10.0),
                    offset: None,
                    units:  Some("%"),
                }))
            },
            101 => {
                Ok(Session::AvgLeftTorqueEffectiveness(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  Some(2.0),
                    offset: None,
                    units:  Some("percent"),
                }))
            },
            102 => {
                Ok(Session::AvgRightTorqueEffectiveness(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  Some(2.0),
                    offset: None,
                    units:  Some("percent"),
                }))
            },
            103 => {
                Ok(Session::AvgLeftPedalSmoothness(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  Some(2.0),
                    offset: None,
                    units:  Some("percent"),
                }))
            },
            104 => {
                Ok(Session::AvgRightPedalSmoothness(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  Some(2.0),
                    offset: None,
                    units:  Some("percent"),
                }))
            },
            105 => {
                Ok(Session::AvgCombinedPedalSmoothness(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  Some(2.0),
                    offset: None,
                    units:  Some("percent"),
                }))
            },
            111 => {
                Ok(Session::SportIndex(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            112 => {
                Ok(Session::TimeStanding(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  Some(1000.0),
                    offset: None,
                    units:  Some("s"),
                }))
            },
            113 => {
                Ok(Session::StandCount(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            114 => {
                Ok(Session::AvgLeftPco(Field {
                    raw_value:  profile::base::Sint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("mm"),
                }))
            },
            115 => {
                Ok(Session::AvgRightPco(Field {
                    raw_value:  profile::base::Sint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("mm"),
                }))
            },
            116 => {
                Ok(Session::AvgLeftPowerPhase(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  Some(0.7111111),
                    offset: None,
                    units:  Some("degrees"),
                }))
            },
            117 => {
                Ok(Session::AvgLeftPowerPhasePeak(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  Some(0.7111111),
                    offset: None,
                    units:  Some("degrees"),
                }))
            },
            118 => {
                Ok(Session::AvgRightPowerPhase(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  Some(0.7111111),
                    offset: None,
                    units:  Some("degrees"),
                }))
            },
            119 => {
                Ok(Session::AvgRightPowerPhasePeak(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  Some(0.7111111),
                    offset: None,
                    units:  Some("degrees"),
                }))
            },
            120 => {
                Ok(Session::AvgPowerPosition(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("watts"),
                }))
            },
            121 => {
                Ok(Session::MaxPowerPosition(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("watts"),
                }))
            },
            122 => {
                Ok(Session::AvgCadencePosition(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("rpm"),
                }))
            },
            123 => {
                Ok(Session::MaxCadencePosition(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("rpm"),
                }))
            },
            124 => {
                Ok(Session::EnhancedAvgSpeed(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  Some(1000.0),
                    offset: None,
                    units:  Some("m/s"),
                }))
            },
            125 => {
                Ok(Session::EnhancedMaxSpeed(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  Some(1000.0),
                    offset: None,
                    units:  Some("m/s"),
                }))
            },
            126 => {
                Ok(Session::EnhancedAvgAltitude(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  Some(5.0),
                    offset: Some(500.0),
                    units:  Some("m"),
                }))
            },
            127 => {
                Ok(Session::EnhancedMinAltitude(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  Some(5.0),
                    offset: Some(500.0),
                    units:  Some("m"),
                }))
            },
            128 => {
                Ok(Session::EnhancedMaxAltitude(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  Some(5.0),
                    offset: Some(500.0),
                    units:  Some("m"),
                }))
            },
            129 => {
                Ok(Session::AvgLevMotorPower(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("watts"),
                }))
            },
            130 => {
                Ok(Session::MaxLevMotorPower(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("watts"),
                }))
            },
            131 => {
                Ok(Session::LevBatteryConsumption(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  Some(2.0),
                    offset: None,
                    units:  Some("percent"),
                }))
            },
            132 => {
                Ok(Session::AvgVerticalRatio(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  Some(100.0),
                    offset: None,
                    units:  Some("percent"),
                }))
            },
            133 => {
                Ok(Session::AvgStanceTimeBalance(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  Some(100.0),
                    offset: None,
                    units:  Some("percent"),
                }))
            },
            134 => {
                Ok(Session::AvgStepLength(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  Some(10.0),
                    offset: None,
                    units:  Some("mm"),
                }))
            },
            137 => {
                Ok(Session::TotalAnaerobicTrainingEffect(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  Some(10.0),
                    offset: None,
                    units:  None,
                }))
            },
            139 => {
                Ok(Session::AvgVam(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  Some(1000.0),
                    offset: None,
                    units:  Some("m/s"),
                }))
            },
            _ => {
                Ok(Session::Unknown {
                    data: buffer.to_vec(),
                    field_def_num,
                })
            },
        }
    }
}
#[derive(Debug,Clone)]
pub enum Lap {
    MessageIndex(Field<profile::types::MessageIndex>),
    #[doc = "Lap end time."]
    Timestamp(Field<profile::types::DateTime>),
    Event(Field<profile::types::Event>),
    EventType(Field<profile::types::EventType>),
    StartTime(Field<profile::types::DateTime>),
    StartPositionLat(Field<profile::base::Sint32>),
    StartPositionLong(Field<profile::base::Sint32>),
    EndPositionLat(Field<profile::base::Sint32>),
    EndPositionLong(Field<profile::base::Sint32>),
    #[doc = "Time (includes pauses)"]
    TotalElapsedTime(Field<profile::base::Uint32>),
    #[doc = "Timer Time (excludes pauses)"]
    TotalTimerTime(Field<profile::base::Uint32>),
    TotalDistance(Field<profile::base::Uint32>),
    TotalCycles(Field<profile::base::Uint32>),
    TotalCalories(Field<profile::base::Uint16>),
    #[doc = "If New Leaf"]
    TotalFatCalories(Field<profile::base::Uint16>),
    AvgSpeed(Field<profile::base::Uint16>),
    MaxSpeed(Field<profile::base::Uint16>),
    AvgHeartRate(Field<profile::base::Uint8>),
    MaxHeartRate(Field<profile::base::Uint8>),
    #[doc = "total_cycles / total_timer_time if non_zero_avg_cadence \
             otherwise total_cycles / total_elapsed_time"]
    AvgCadence(Field<profile::base::Uint8>),
    MaxCadence(Field<profile::base::Uint8>),
    #[doc = "total_power / total_timer_time if non_zero_avg_power otherwise \
             total_power / total_elapsed_time"]
    AvgPower(Field<profile::base::Uint16>),
    MaxPower(Field<profile::base::Uint16>),
    TotalAscent(Field<profile::base::Uint16>),
    TotalDescent(Field<profile::base::Uint16>),
    Intensity(Field<profile::types::Intensity>),
    LapTrigger(Field<profile::types::LapTrigger>),
    Sport(Field<profile::types::Sport>),
    EventGroup(Field<profile::base::Uint8>),
    #[doc = "# of lengths of swim pool"]
    NumLengths(Field<profile::base::Uint16>),
    NormalizedPower(Field<profile::base::Uint16>),
    LeftRightBalance(Field<profile::types::LeftRightBalance100>),
    FirstLengthIndex(Field<profile::base::Uint16>),
    AvgStrokeDistance(Field<profile::base::Uint16>),
    SwimStroke(Field<profile::types::SwimStroke>),
    SubSport(Field<profile::types::SubSport>),
    #[doc = "# of active lengths of swim pool"]
    NumActiveLengths(Field<profile::base::Uint16>),
    TotalWork(Field<profile::base::Uint32>),
    AvgAltitude(Field<profile::base::Uint16>),
    MaxAltitude(Field<profile::base::Uint16>),
    GpsAccuracy(Field<profile::base::Uint8>),
    AvgGrade(Field<profile::base::Sint16>),
    AvgPosGrade(Field<profile::base::Sint16>),
    AvgNegGrade(Field<profile::base::Sint16>),
    MaxPosGrade(Field<profile::base::Sint16>),
    MaxNegGrade(Field<profile::base::Sint16>),
    AvgTemperature(Field<profile::base::Sint8>),
    MaxTemperature(Field<profile::base::Sint8>),
    TotalMovingTime(Field<profile::base::Uint32>),
    AvgPosVerticalSpeed(Field<profile::base::Sint16>),
    AvgNegVerticalSpeed(Field<profile::base::Sint16>),
    MaxPosVerticalSpeed(Field<profile::base::Sint16>),
    MaxNegVerticalSpeed(Field<profile::base::Sint16>),
    TimeInHrZone(Field<profile::base::Uint32>),
    TimeInSpeedZone(Field<profile::base::Uint32>),
    TimeInCadenceZone(Field<profile::base::Uint32>),
    TimeInPowerZone(Field<profile::base::Uint32>),
    RepetitionNum(Field<profile::base::Uint16>),
    MinAltitude(Field<profile::base::Uint16>),
    MinHeartRate(Field<profile::base::Uint8>),
    WktStepIndex(Field<profile::types::MessageIndex>),
    OpponentScore(Field<profile::base::Uint16>),
    #[doc = "stroke_type enum used as the index"]
    StrokeCount(Field<profile::base::Uint16>),
    #[doc = "zone number used as the index"]
    ZoneCount(Field<profile::base::Uint16>),
    AvgVerticalOscillation(Field<profile::base::Uint16>),
    AvgStanceTimePercent(Field<profile::base::Uint16>),
    AvgStanceTime(Field<profile::base::Uint16>),
    #[doc = "fractional part of the avg_cadence"]
    AvgFractionalCadence(Field<profile::base::Uint8>),
    #[doc = "fractional part of the max_cadence"]
    MaxFractionalCadence(Field<profile::base::Uint8>),
    #[doc = "fractional part of the total_cycles"]
    TotalFractionalCycles(Field<profile::base::Uint8>),
    PlayerScore(Field<profile::base::Uint16>),
    #[doc = "Avg saturated and unsaturated hemoglobin"]
    AvgTotalHemoglobinConc(Field<profile::base::Uint16>),
    #[doc = "Min saturated and unsaturated hemoglobin"]
    MinTotalHemoglobinConc(Field<profile::base::Uint16>),
    #[doc = "Max saturated and unsaturated hemoglobin"]
    MaxTotalHemoglobinConc(Field<profile::base::Uint16>),
    #[doc = "Avg percentage of hemoglobin saturated with oxygen"]
    AvgSaturatedHemoglobinPercent(Field<profile::base::Uint16>),
    #[doc = "Min percentage of hemoglobin saturated with oxygen"]
    MinSaturatedHemoglobinPercent(Field<profile::base::Uint16>),
    #[doc = "Max percentage of hemoglobin saturated with oxygen"]
    MaxSaturatedHemoglobinPercent(Field<profile::base::Uint16>),
    AvgLeftTorqueEffectiveness(Field<profile::base::Uint8>),
    AvgRightTorqueEffectiveness(Field<profile::base::Uint8>),
    AvgLeftPedalSmoothness(Field<profile::base::Uint8>),
    AvgRightPedalSmoothness(Field<profile::base::Uint8>),
    AvgCombinedPedalSmoothness(Field<profile::base::Uint8>),
    #[doc = "Total time spent in the standing position"]
    TimeStanding(Field<profile::base::Uint32>),
    #[doc = "Number of transitions to the standing state"]
    StandCount(Field<profile::base::Uint16>),
    #[doc = "Average left platform center offset"]
    AvgLeftPco(Field<profile::base::Sint8>),
    #[doc = "Average right platform center offset"]
    AvgRightPco(Field<profile::base::Sint8>),
    #[doc = "Average left power phase angles. Data raw_value indexes defined by \
             power_phase_type."]
    AvgLeftPowerPhase(Field<profile::base::Uint8>),
    #[doc = "Average left power phase peak angles. Data raw_value indexes  \
             defined by power_phase_type."]
    AvgLeftPowerPhasePeak(Field<profile::base::Uint8>),
    #[doc = "Average right power phase angles. Data raw_value indexes defined by \
             power_phase_type."]
    AvgRightPowerPhase(Field<profile::base::Uint8>),
    #[doc = "Average right power phase peak angles. Data raw_value indexes  \
             defined by power_phase_type."]
    AvgRightPowerPhasePeak(Field<profile::base::Uint8>),
    #[doc = "Average power by position. Data raw_value indexes defined by \
             rider_position_type."]
    AvgPowerPosition(Field<profile::base::Uint16>),
    #[doc = "Maximum power by position. Data raw_value indexes defined by \
             rider_position_type."]
    MaxPowerPosition(Field<profile::base::Uint16>),
    #[doc = "Average cadence by position. Data raw_value indexes defined by \
             rider_position_type."]
    AvgCadencePosition(Field<profile::base::Uint8>),
    #[doc = "Maximum cadence by position. Data raw_value indexes defined by \
             rider_position_type."]
    MaxCadencePosition(Field<profile::base::Uint8>),
    EnhancedAvgSpeed(Field<profile::base::Uint32>),
    EnhancedMaxSpeed(Field<profile::base::Uint32>),
    EnhancedAvgAltitude(Field<profile::base::Uint32>),
    EnhancedMinAltitude(Field<profile::base::Uint32>),
    EnhancedMaxAltitude(Field<profile::base::Uint32>),
    #[doc = "lev average motor power during lap"]
    AvgLevMotorPower(Field<profile::base::Uint16>),
    #[doc = "lev maximum motor power during lap"]
    MaxLevMotorPower(Field<profile::base::Uint16>),
    #[doc = "lev battery consumption during lap"]
    LevBatteryConsumption(Field<profile::base::Uint8>),
    AvgVerticalRatio(Field<profile::base::Uint16>),
    AvgStanceTimeBalance(Field<profile::base::Uint16>),
    AvgStepLength(Field<profile::base::Uint16>),
    AvgVam(Field<profile::base::Uint16>),
    Unknown {
        data:          Vec<u8>,
        field_def_num: u8,
    },
}
impl Lap {
    pub(crate) fn decode<T: ByteOrder>(
        buffer: &[u8],
        field_def_num: u8,
    ) -> error::Result<Self> {
        match field_def_num {
            254 => {
                Ok(Lap::MessageIndex(Field {
                    raw_value:  profile::types::MessageIndex::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            253 => {
                Ok(Lap::Timestamp(Field {
                    raw_value:  profile::types::DateTime::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("s"),
                }))
            },
            0 => {
                Ok(Lap::Event(Field {
                    raw_value:  profile::types::Event::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            1 => {
                Ok(Lap::EventType(Field {
                    raw_value:  profile::types::EventType::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            2 => {
                Ok(Lap::StartTime(Field {
                    raw_value:  profile::types::DateTime::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            3 => {
                Ok(Lap::StartPositionLat(Field {
                    raw_value:  profile::base::Sint32::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("semicircles"),
                }))
            },
            4 => {
                Ok(Lap::StartPositionLong(Field {
                    raw_value:  profile::base::Sint32::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("semicircles"),
                }))
            },
            5 => {
                Ok(Lap::EndPositionLat(Field {
                    raw_value:  profile::base::Sint32::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("semicircles"),
                }))
            },
            6 => {
                Ok(Lap::EndPositionLong(Field {
                    raw_value:  profile::base::Sint32::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("semicircles"),
                }))
            },
            7 => {
                Ok(Lap::TotalElapsedTime(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  Some(1000.0),
                    offset: None,
                    units:  Some("s"),
                }))
            },
            8 => {
                Ok(Lap::TotalTimerTime(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  Some(1000.0),
                    offset: None,
                    units:  Some("s"),
                }))
            },
            9 => {
                Ok(Lap::TotalDistance(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  Some(100.0),
                    offset: None,
                    units:  Some("m"),
                }))
            },
            10 => {
                Ok(Lap::TotalCycles(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("cycles"),
                }))
            },
            11 => {
                Ok(Lap::TotalCalories(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("kcal"),
                }))
            },
            12 => {
                Ok(Lap::TotalFatCalories(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("kcal"),
                }))
            },
            13 => {
                Ok(Lap::AvgSpeed(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  Some(1000.0),
                    offset: None,
                    units:  Some("m/s"),
                }))
            },
            14 => {
                Ok(Lap::MaxSpeed(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  Some(1000.0),
                    offset: None,
                    units:  Some("m/s"),
                }))
            },
            15 => {
                Ok(Lap::AvgHeartRate(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("bpm"),
                }))
            },
            16 => {
                Ok(Lap::MaxHeartRate(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("bpm"),
                }))
            },
            17 => {
                Ok(Lap::AvgCadence(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("rpm"),
                }))
            },
            18 => {
                Ok(Lap::MaxCadence(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("rpm"),
                }))
            },
            19 => {
                Ok(Lap::AvgPower(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("watts"),
                }))
            },
            20 => {
                Ok(Lap::MaxPower(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("watts"),
                }))
            },
            21 => {
                Ok(Lap::TotalAscent(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("m"),
                }))
            },
            22 => {
                Ok(Lap::TotalDescent(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("m"),
                }))
            },
            23 => {
                Ok(Lap::Intensity(Field {
                    raw_value:  profile::types::Intensity::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            24 => {
                Ok(Lap::LapTrigger(Field {
                    raw_value:  profile::types::LapTrigger::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            25 => {
                Ok(Lap::Sport(Field {
                    raw_value:  profile::types::Sport::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            26 => {
                Ok(Lap::EventGroup(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            32 => {
                Ok(Lap::NumLengths(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("lengths"),
                }))
            },
            33 => {
                Ok(Lap::NormalizedPower(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("watts"),
                }))
            },
            34 => {
                Ok(Lap::LeftRightBalance(Field {
                    raw_value:  profile::types::LeftRightBalance100::decode::<T>(
                        buffer,
                    )?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            35 => {
                Ok(Lap::FirstLengthIndex(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            37 => {
                Ok(Lap::AvgStrokeDistance(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  Some(100.0),
                    offset: None,
                    units:  Some("m"),
                }))
            },
            38 => {
                Ok(Lap::SwimStroke(Field {
                    raw_value:  profile::types::SwimStroke::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            39 => {
                Ok(Lap::SubSport(Field {
                    raw_value:  profile::types::SubSport::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            40 => {
                Ok(Lap::NumActiveLengths(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("lengths"),
                }))
            },
            41 => {
                Ok(Lap::TotalWork(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("J"),
                }))
            },
            42 => {
                Ok(Lap::AvgAltitude(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  Some(5.0),
                    offset: Some(500.0),
                    units:  Some("m"),
                }))
            },
            43 => {
                Ok(Lap::MaxAltitude(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  Some(5.0),
                    offset: Some(500.0),
                    units:  Some("m"),
                }))
            },
            44 => {
                Ok(Lap::GpsAccuracy(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("m"),
                }))
            },
            45 => {
                Ok(Lap::AvgGrade(Field {
                    raw_value:  profile::base::Sint16::decode::<T>(buffer)?,
                    scale:  Some(100.0),
                    offset: None,
                    units:  Some("%"),
                }))
            },
            46 => {
                Ok(Lap::AvgPosGrade(Field {
                    raw_value:  profile::base::Sint16::decode::<T>(buffer)?,
                    scale:  Some(100.0),
                    offset: None,
                    units:  Some("%"),
                }))
            },
            47 => {
                Ok(Lap::AvgNegGrade(Field {
                    raw_value:  profile::base::Sint16::decode::<T>(buffer)?,
                    scale:  Some(100.0),
                    offset: None,
                    units:  Some("%"),
                }))
            },
            48 => {
                Ok(Lap::MaxPosGrade(Field {
                    raw_value:  profile::base::Sint16::decode::<T>(buffer)?,
                    scale:  Some(100.0),
                    offset: None,
                    units:  Some("%"),
                }))
            },
            49 => {
                Ok(Lap::MaxNegGrade(Field {
                    raw_value:  profile::base::Sint16::decode::<T>(buffer)?,
                    scale:  Some(100.0),
                    offset: None,
                    units:  Some("%"),
                }))
            },
            50 => {
                Ok(Lap::AvgTemperature(Field {
                    raw_value:  profile::base::Sint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("C"),
                }))
            },
            51 => {
                Ok(Lap::MaxTemperature(Field {
                    raw_value:  profile::base::Sint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("C"),
                }))
            },
            52 => {
                Ok(Lap::TotalMovingTime(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  Some(1000.0),
                    offset: None,
                    units:  Some("s"),
                }))
            },
            53 => {
                Ok(Lap::AvgPosVerticalSpeed(Field {
                    raw_value:  profile::base::Sint16::decode::<T>(buffer)?,
                    scale:  Some(1000.0),
                    offset: None,
                    units:  Some("m/s"),
                }))
            },
            54 => {
                Ok(Lap::AvgNegVerticalSpeed(Field {
                    raw_value:  profile::base::Sint16::decode::<T>(buffer)?,
                    scale:  Some(1000.0),
                    offset: None,
                    units:  Some("m/s"),
                }))
            },
            55 => {
                Ok(Lap::MaxPosVerticalSpeed(Field {
                    raw_value:  profile::base::Sint16::decode::<T>(buffer)?,
                    scale:  Some(1000.0),
                    offset: None,
                    units:  Some("m/s"),
                }))
            },
            56 => {
                Ok(Lap::MaxNegVerticalSpeed(Field {
                    raw_value:  profile::base::Sint16::decode::<T>(buffer)?,
                    scale:  Some(1000.0),
                    offset: None,
                    units:  Some("m/s"),
                }))
            },
            57 => {
                Ok(Lap::TimeInHrZone(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  Some(1000.0),
                    offset: None,
                    units:  Some("s"),
                }))
            },
            58 => {
                Ok(Lap::TimeInSpeedZone(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  Some(1000.0),
                    offset: None,
                    units:  Some("s"),
                }))
            },
            59 => {
                Ok(Lap::TimeInCadenceZone(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  Some(1000.0),
                    offset: None,
                    units:  Some("s"),
                }))
            },
            60 => {
                Ok(Lap::TimeInPowerZone(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  Some(1000.0),
                    offset: None,
                    units:  Some("s"),
                }))
            },
            61 => {
                Ok(Lap::RepetitionNum(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            62 => {
                Ok(Lap::MinAltitude(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  Some(5.0),
                    offset: Some(500.0),
                    units:  Some("m"),
                }))
            },
            63 => {
                Ok(Lap::MinHeartRate(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("bpm"),
                }))
            },
            71 => {
                Ok(Lap::WktStepIndex(Field {
                    raw_value:  profile::types::MessageIndex::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            74 => {
                Ok(Lap::OpponentScore(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            75 => {
                Ok(Lap::StrokeCount(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("counts"),
                }))
            },
            76 => {
                Ok(Lap::ZoneCount(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("counts"),
                }))
            },
            77 => {
                Ok(Lap::AvgVerticalOscillation(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  Some(10.0),
                    offset: None,
                    units:  Some("mm"),
                }))
            },
            78 => {
                Ok(Lap::AvgStanceTimePercent(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  Some(100.0),
                    offset: None,
                    units:  Some("percent"),
                }))
            },
            79 => {
                Ok(Lap::AvgStanceTime(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  Some(10.0),
                    offset: None,
                    units:  Some("ms"),
                }))
            },
            80 => {
                Ok(Lap::AvgFractionalCadence(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  Some(128.0),
                    offset: None,
                    units:  Some("rpm"),
                }))
            },
            81 => {
                Ok(Lap::MaxFractionalCadence(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  Some(128.0),
                    offset: None,
                    units:  Some("rpm"),
                }))
            },
            82 => {
                Ok(Lap::TotalFractionalCycles(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  Some(128.0),
                    offset: None,
                    units:  Some("cycles"),
                }))
            },
            83 => {
                Ok(Lap::PlayerScore(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            84 => {
                Ok(Lap::AvgTotalHemoglobinConc(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  Some(100.0),
                    offset: None,
                    units:  Some("g/dL"),
                }))
            },
            85 => {
                Ok(Lap::MinTotalHemoglobinConc(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  Some(100.0),
                    offset: None,
                    units:  Some("g/dL"),
                }))
            },
            86 => {
                Ok(Lap::MaxTotalHemoglobinConc(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  Some(100.0),
                    offset: None,
                    units:  Some("g/dL"),
                }))
            },
            87 => {
                Ok(Lap::AvgSaturatedHemoglobinPercent(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  Some(10.0),
                    offset: None,
                    units:  Some("%"),
                }))
            },
            88 => {
                Ok(Lap::MinSaturatedHemoglobinPercent(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  Some(10.0),
                    offset: None,
                    units:  Some("%"),
                }))
            },
            89 => {
                Ok(Lap::MaxSaturatedHemoglobinPercent(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  Some(10.0),
                    offset: None,
                    units:  Some("%"),
                }))
            },
            91 => {
                Ok(Lap::AvgLeftTorqueEffectiveness(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  Some(2.0),
                    offset: None,
                    units:  Some("percent"),
                }))
            },
            92 => {
                Ok(Lap::AvgRightTorqueEffectiveness(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  Some(2.0),
                    offset: None,
                    units:  Some("percent"),
                }))
            },
            93 => {
                Ok(Lap::AvgLeftPedalSmoothness(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  Some(2.0),
                    offset: None,
                    units:  Some("percent"),
                }))
            },
            94 => {
                Ok(Lap::AvgRightPedalSmoothness(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  Some(2.0),
                    offset: None,
                    units:  Some("percent"),
                }))
            },
            95 => {
                Ok(Lap::AvgCombinedPedalSmoothness(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  Some(2.0),
                    offset: None,
                    units:  Some("percent"),
                }))
            },
            98 => {
                Ok(Lap::TimeStanding(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  Some(1000.0),
                    offset: None,
                    units:  Some("s"),
                }))
            },
            99 => {
                Ok(Lap::StandCount(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            100 => {
                Ok(Lap::AvgLeftPco(Field {
                    raw_value:  profile::base::Sint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("mm"),
                }))
            },
            101 => {
                Ok(Lap::AvgRightPco(Field {
                    raw_value:  profile::base::Sint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("mm"),
                }))
            },
            102 => {
                Ok(Lap::AvgLeftPowerPhase(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  Some(0.7111111),
                    offset: None,
                    units:  Some("degrees"),
                }))
            },
            103 => {
                Ok(Lap::AvgLeftPowerPhasePeak(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  Some(0.7111111),
                    offset: None,
                    units:  Some("degrees"),
                }))
            },
            104 => {
                Ok(Lap::AvgRightPowerPhase(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  Some(0.7111111),
                    offset: None,
                    units:  Some("degrees"),
                }))
            },
            105 => {
                Ok(Lap::AvgRightPowerPhasePeak(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  Some(0.7111111),
                    offset: None,
                    units:  Some("degrees"),
                }))
            },
            106 => {
                Ok(Lap::AvgPowerPosition(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("watts"),
                }))
            },
            107 => {
                Ok(Lap::MaxPowerPosition(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("watts"),
                }))
            },
            108 => {
                Ok(Lap::AvgCadencePosition(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("rpm"),
                }))
            },
            109 => {
                Ok(Lap::MaxCadencePosition(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("rpm"),
                }))
            },
            110 => {
                Ok(Lap::EnhancedAvgSpeed(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  Some(1000.0),
                    offset: None,
                    units:  Some("m/s"),
                }))
            },
            111 => {
                Ok(Lap::EnhancedMaxSpeed(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  Some(1000.0),
                    offset: None,
                    units:  Some("m/s"),
                }))
            },
            112 => {
                Ok(Lap::EnhancedAvgAltitude(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  Some(5.0),
                    offset: Some(500.0),
                    units:  Some("m"),
                }))
            },
            113 => {
                Ok(Lap::EnhancedMinAltitude(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  Some(5.0),
                    offset: Some(500.0),
                    units:  Some("m"),
                }))
            },
            114 => {
                Ok(Lap::EnhancedMaxAltitude(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  Some(5.0),
                    offset: Some(500.0),
                    units:  Some("m"),
                }))
            },
            115 => {
                Ok(Lap::AvgLevMotorPower(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("watts"),
                }))
            },
            116 => {
                Ok(Lap::MaxLevMotorPower(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("watts"),
                }))
            },
            117 => {
                Ok(Lap::LevBatteryConsumption(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  Some(2.0),
                    offset: None,
                    units:  Some("percent"),
                }))
            },
            118 => {
                Ok(Lap::AvgVerticalRatio(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  Some(100.0),
                    offset: None,
                    units:  Some("percent"),
                }))
            },
            119 => {
                Ok(Lap::AvgStanceTimeBalance(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  Some(100.0),
                    offset: None,
                    units:  Some("percent"),
                }))
            },
            120 => {
                Ok(Lap::AvgStepLength(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  Some(10.0),
                    offset: None,
                    units:  Some("mm"),
                }))
            },
            121 => {
                Ok(Lap::AvgVam(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  Some(1000.0),
                    offset: None,
                    units:  Some("m/s"),
                }))
            },
            _ => {
                Ok(Lap::Unknown {
                    data: buffer.to_vec(),
                    field_def_num,
                })
            },
        }
    }
}
#[derive(Debug,Clone)]
pub enum Length {
    MessageIndex(Field<profile::types::MessageIndex>),
    Timestamp(Field<profile::types::DateTime>),
    Event(Field<profile::types::Event>),
    EventType(Field<profile::types::EventType>),
    StartTime(Field<profile::types::DateTime>),
    TotalElapsedTime(Field<profile::base::Uint32>),
    TotalTimerTime(Field<profile::base::Uint32>),
    TotalStrokes(Field<profile::base::Uint16>),
    AvgSpeed(Field<profile::base::Uint16>),
    SwimStroke(Field<profile::types::SwimStroke>),
    AvgSwimmingCadence(Field<profile::base::Uint8>),
    EventGroup(Field<profile::base::Uint8>),
    TotalCalories(Field<profile::base::Uint16>),
    LengthType(Field<profile::types::LengthType>),
    PlayerScore(Field<profile::base::Uint16>),
    OpponentScore(Field<profile::base::Uint16>),
    #[doc = "stroke_type enum used as the index"]
    StrokeCount(Field<profile::base::Uint16>),
    #[doc = "zone number used as the index"]
    ZoneCount(Field<profile::base::Uint16>),
    Unknown {
        data:          Vec<u8>,
        field_def_num: u8,
    },
}
impl Length {
    pub(crate) fn decode<T: ByteOrder>(
        buffer: &[u8],
        field_def_num: u8,
    ) -> error::Result<Self> {
        match field_def_num {
            254 => {
                Ok(Length::MessageIndex(Field {
                    raw_value:  profile::types::MessageIndex::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            253 => {
                Ok(Length::Timestamp(Field {
                    raw_value:  profile::types::DateTime::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            0 => {
                Ok(Length::Event(Field {
                    raw_value:  profile::types::Event::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            1 => {
                Ok(Length::EventType(Field {
                    raw_value:  profile::types::EventType::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            2 => {
                Ok(Length::StartTime(Field {
                    raw_value:  profile::types::DateTime::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            3 => {
                Ok(Length::TotalElapsedTime(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  Some(1000.0),
                    offset: None,
                    units:  Some("s"),
                }))
            },
            4 => {
                Ok(Length::TotalTimerTime(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  Some(1000.0),
                    offset: None,
                    units:  Some("s"),
                }))
            },
            5 => {
                Ok(Length::TotalStrokes(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("strokes"),
                }))
            },
            6 => {
                Ok(Length::AvgSpeed(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  Some(1000.0),
                    offset: None,
                    units:  Some("m/s"),
                }))
            },
            7 => {
                Ok(Length::SwimStroke(Field {
                    raw_value:  profile::types::SwimStroke::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("swim_stroke"),
                }))
            },
            9 => {
                Ok(Length::AvgSwimmingCadence(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("strokes/min"),
                }))
            },
            10 => {
                Ok(Length::EventGroup(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            11 => {
                Ok(Length::TotalCalories(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("kcal"),
                }))
            },
            12 => {
                Ok(Length::LengthType(Field {
                    raw_value:  profile::types::LengthType::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            18 => {
                Ok(Length::PlayerScore(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            19 => {
                Ok(Length::OpponentScore(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            20 => {
                Ok(Length::StrokeCount(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("counts"),
                }))
            },
            21 => {
                Ok(Length::ZoneCount(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("counts"),
                }))
            },
            _ => {
                Ok(Length::Unknown {
                    data: buffer.to_vec(),
                    field_def_num,
                })
            },
        }
    }
}
#[derive(Debug,Clone)]
pub enum Record {
    Timestamp(Field<profile::types::DateTime>),
    PositionLat(Field<profile::base::Sint32>),
    PositionLong(Field<profile::base::Sint32>),
    Altitude(Field<profile::base::Uint16>),
    HeartRate(Field<profile::base::Uint8>),
    Cadence(Field<profile::base::Uint8>),
    Distance(Field<profile::base::Uint32>),
    Speed(Field<profile::base::Uint16>),
    Power(Field<profile::base::Uint16>),
    CompressedSpeedDistance(Field<profile::base::Bytes>),
    Grade(Field<profile::base::Sint16>),
    #[doc = "Relative. 0 is none  254 is Max."]
    Resistance(Field<profile::base::Uint8>),
    TimeFromCourse(Field<profile::base::Sint32>),
    CycleLength(Field<profile::base::Uint8>),
    Temperature(Field<profile::base::Sint8>),
    #[doc = "Speed at 1s intervals.  Timestamp field indicates time of last \
             array element."]
    Speed1S(Field<profile::base::Uint8>),
    Cycles(Field<profile::base::Uint8>),
    TotalCycles(Field<profile::base::Uint32>),
    CompressedAccumulatedPower(Field<profile::base::Uint16>),
    AccumulatedPower(Field<profile::base::Uint32>),
    LeftRightBalance(Field<profile::types::LeftRightBalance>),
    GpsAccuracy(Field<profile::base::Uint8>),
    VerticalSpeed(Field<profile::base::Sint16>),
    Calories(Field<profile::base::Uint16>),
    VerticalOscillation(Field<profile::base::Uint16>),
    StanceTimePercent(Field<profile::base::Uint16>),
    StanceTime(Field<profile::base::Uint16>),
    ActivityType(Field<profile::types::ActivityType>),
    LeftTorqueEffectiveness(Field<profile::base::Uint8>),
    RightTorqueEffectiveness(Field<profile::base::Uint8>),
    LeftPedalSmoothness(Field<profile::base::Uint8>),
    RightPedalSmoothness(Field<profile::base::Uint8>),
    CombinedPedalSmoothness(Field<profile::base::Uint8>),
    Time128(Field<profile::base::Uint8>),
    StrokeType(Field<profile::types::StrokeType>),
    Zone(Field<profile::base::Uint8>),
    BallSpeed(Field<profile::base::Uint16>),
    #[doc = "Log cadence and fractional cadence for backwards compatability"]
    Cadence256(Field<profile::base::Uint16>),
    FractionalCadence(Field<profile::base::Uint8>),
    #[doc = "Total saturated and unsaturated hemoglobin"]
    TotalHemoglobinConc(Field<profile::base::Uint16>),
    #[doc = "Min saturated and unsaturated hemoglobin"]
    TotalHemoglobinConcMin(Field<profile::base::Uint16>),
    #[doc = "Max saturated and unsaturated hemoglobin"]
    TotalHemoglobinConcMax(Field<profile::base::Uint16>),
    #[doc = "Percentage of hemoglobin saturated with oxygen"]
    SaturatedHemoglobinPercent(Field<profile::base::Uint16>),
    #[doc = "Min percentage of hemoglobin saturated with oxygen"]
    SaturatedHemoglobinPercentMin(Field<profile::base::Uint16>),
    #[doc = "Max percentage of hemoglobin saturated with oxygen"]
    SaturatedHemoglobinPercentMax(Field<profile::base::Uint16>),
    DeviceIndex(Field<profile::types::DeviceIndex>),
    #[doc = "Left platform center offset"]
    LeftPco(Field<profile::base::Sint8>),
    #[doc = "Right platform center offset"]
    RightPco(Field<profile::base::Sint8>),
    #[doc = "Left power phase angles. Data raw_value indexes defined by \
             power_phase_type."]
    LeftPowerPhase(Field<profile::base::Uint8>),
    #[doc = "Left power phase peak angles. Data raw_value indexes defined by \
             power_phase_type."]
    LeftPowerPhasePeak(Field<profile::base::Uint8>),
    #[doc = "Right power phase angles. Data raw_value indexes defined by \
             power_phase_type."]
    RightPowerPhase(Field<profile::base::Uint8>),
    #[doc = "Right power phase peak angles. Data raw_value indexes defined by \
             power_phase_type."]
    RightPowerPhasePeak(Field<profile::base::Uint8>),
    EnhancedSpeed(Field<profile::base::Uint32>),
    EnhancedAltitude(Field<profile::base::Uint32>),
    #[doc = "lev battery state of charge"]
    BatterySoc(Field<profile::base::Uint8>),
    #[doc = "lev motor power"]
    MotorPower(Field<profile::base::Uint16>),
    VerticalRatio(Field<profile::base::Uint16>),
    StanceTimeBalance(Field<profile::base::Uint16>),
    StepLength(Field<profile::base::Uint16>),
    #[doc = "Includes atmospheric pressure"]
    AbsolutePressure(Field<profile::base::Uint32>),
    #[doc = "0 if above water"]
    Depth(Field<profile::base::Uint32>),
    #[doc = "0 if above water"]
    NextStopDepth(Field<profile::base::Uint32>),
    NextStopTime(Field<profile::base::Uint32>),
    TimeToSurface(Field<profile::base::Uint32>),
    NdlTime(Field<profile::base::Uint32>),
    CnsLoad(Field<profile::base::Uint8>),
    N2Load(Field<profile::base::Uint16>),
    Unknown {
        data:          Vec<u8>,
        field_def_num: u8,
    },
}
impl Record {
    pub(crate) fn decode<T: ByteOrder>(
        buffer: &[u8],
        field_def_num: u8,
    ) -> error::Result<Self> {
        match field_def_num {
            253 => {
                Ok(Record::Timestamp(Field {
                    raw_value:  profile::types::DateTime::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("s"),
                }))
            },
            0 => {
                Ok(Record::PositionLat(Field {
                    raw_value:  profile::base::Sint32::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("semicircles"),
                }))
            },
            1 => {
                Ok(Record::PositionLong(Field {
                    raw_value:  profile::base::Sint32::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("semicircles"),
                }))
            },
            2 => {
                Ok(Record::Altitude(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  Some(5.0),
                    offset: Some(500.0),
                    units:  Some("m"),
                }))
            },
            3 => {
                Ok(Record::HeartRate(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("bpm"),
                }))
            },
            4 => {
                Ok(Record::Cadence(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("rpm"),
                }))
            },
            5 => {
                Ok(Record::Distance(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  Some(100.0),
                    offset: None,
                    units:  Some("m"),
                }))
            },
            6 => {
                Ok(Record::Speed(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  Some(1000.0),
                    offset: None,
                    units:  Some("m/s"),
                }))
            },
            7 => {
                Ok(Record::Power(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("watts"),
                }))
            },
            8 => {
                Ok(Record::CompressedSpeedDistance(Field {
                    raw_value:  profile::base::Bytes::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("m/s,\r\nm"),
                }))
            },
            9 => {
                Ok(Record::Grade(Field {
                    raw_value:  profile::base::Sint16::decode::<T>(buffer)?,
                    scale:  Some(100.0),
                    offset: None,
                    units:  Some("%"),
                }))
            },
            10 => {
                Ok(Record::Resistance(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            11 => {
                Ok(Record::TimeFromCourse(Field {
                    raw_value:  profile::base::Sint32::decode::<T>(buffer)?,
                    scale:  Some(1000.0),
                    offset: None,
                    units:  Some("s"),
                }))
            },
            12 => {
                Ok(Record::CycleLength(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  Some(100.0),
                    offset: None,
                    units:  Some("m"),
                }))
            },
            13 => {
                Ok(Record::Temperature(Field {
                    raw_value:  profile::base::Sint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("C"),
                }))
            },
            17 => {
                Ok(Record::Speed1S(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  Some(16.0),
                    offset: None,
                    units:  Some("m/s"),
                }))
            },
            18 => {
                Ok(Record::Cycles(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("cycles"),
                }))
            },
            19 => {
                Ok(Record::TotalCycles(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("cycles"),
                }))
            },
            28 => {
                Ok(Record::CompressedAccumulatedPower(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("watts"),
                }))
            },
            29 => {
                Ok(Record::AccumulatedPower(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("watts"),
                }))
            },
            30 => {
                Ok(Record::LeftRightBalance(Field {
                    raw_value:  profile::types::LeftRightBalance::decode::<T>(
                        buffer,
                    )?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            31 => {
                Ok(Record::GpsAccuracy(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("m"),
                }))
            },
            32 => {
                Ok(Record::VerticalSpeed(Field {
                    raw_value:  profile::base::Sint16::decode::<T>(buffer)?,
                    scale:  Some(1000.0),
                    offset: None,
                    units:  Some("m/s"),
                }))
            },
            33 => {
                Ok(Record::Calories(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("kcal"),
                }))
            },
            39 => {
                Ok(Record::VerticalOscillation(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  Some(10.0),
                    offset: None,
                    units:  Some("mm"),
                }))
            },
            40 => {
                Ok(Record::StanceTimePercent(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  Some(100.0),
                    offset: None,
                    units:  Some("percent"),
                }))
            },
            41 => {
                Ok(Record::StanceTime(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  Some(10.0),
                    offset: None,
                    units:  Some("ms"),
                }))
            },
            42 => {
                Ok(Record::ActivityType(Field {
                    raw_value:  profile::types::ActivityType::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            43 => {
                Ok(Record::LeftTorqueEffectiveness(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  Some(2.0),
                    offset: None,
                    units:  Some("percent"),
                }))
            },
            44 => {
                Ok(Record::RightTorqueEffectiveness(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  Some(2.0),
                    offset: None,
                    units:  Some("percent"),
                }))
            },
            45 => {
                Ok(Record::LeftPedalSmoothness(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  Some(2.0),
                    offset: None,
                    units:  Some("percent"),
                }))
            },
            46 => {
                Ok(Record::RightPedalSmoothness(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  Some(2.0),
                    offset: None,
                    units:  Some("percent"),
                }))
            },
            47 => {
                Ok(Record::CombinedPedalSmoothness(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  Some(2.0),
                    offset: None,
                    units:  Some("percent"),
                }))
            },
            48 => {
                Ok(Record::Time128(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  Some(128.0),
                    offset: None,
                    units:  Some("s"),
                }))
            },
            49 => {
                Ok(Record::StrokeType(Field {
                    raw_value:  profile::types::StrokeType::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            50 => {
                Ok(Record::Zone(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            51 => {
                Ok(Record::BallSpeed(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  Some(100.0),
                    offset: None,
                    units:  Some("m/s"),
                }))
            },
            52 => {
                Ok(Record::Cadence256(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  Some(256.0),
                    offset: None,
                    units:  Some("rpm"),
                }))
            },
            53 => {
                Ok(Record::FractionalCadence(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  Some(128.0),
                    offset: None,
                    units:  Some("rpm"),
                }))
            },
            54 => {
                Ok(Record::TotalHemoglobinConc(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  Some(100.0),
                    offset: None,
                    units:  Some("g/dL"),
                }))
            },
            55 => {
                Ok(Record::TotalHemoglobinConcMin(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  Some(100.0),
                    offset: None,
                    units:  Some("g/dL"),
                }))
            },
            56 => {
                Ok(Record::TotalHemoglobinConcMax(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  Some(100.0),
                    offset: None,
                    units:  Some("g/dL"),
                }))
            },
            57 => {
                Ok(Record::SaturatedHemoglobinPercent(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  Some(10.0),
                    offset: None,
                    units:  Some("%"),
                }))
            },
            58 => {
                Ok(Record::SaturatedHemoglobinPercentMin(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  Some(10.0),
                    offset: None,
                    units:  Some("%"),
                }))
            },
            59 => {
                Ok(Record::SaturatedHemoglobinPercentMax(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  Some(10.0),
                    offset: None,
                    units:  Some("%"),
                }))
            },
            62 => {
                Ok(Record::DeviceIndex(Field {
                    raw_value:  profile::types::DeviceIndex::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            67 => {
                Ok(Record::LeftPco(Field {
                    raw_value:  profile::base::Sint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("mm"),
                }))
            },
            68 => {
                Ok(Record::RightPco(Field {
                    raw_value:  profile::base::Sint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("mm"),
                }))
            },
            69 => {
                Ok(Record::LeftPowerPhase(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  Some(0.7111111),
                    offset: None,
                    units:  Some("degrees"),
                }))
            },
            70 => {
                Ok(Record::LeftPowerPhasePeak(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  Some(0.7111111),
                    offset: None,
                    units:  Some("degrees"),
                }))
            },
            71 => {
                Ok(Record::RightPowerPhase(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  Some(0.7111111),
                    offset: None,
                    units:  Some("degrees"),
                }))
            },
            72 => {
                Ok(Record::RightPowerPhasePeak(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  Some(0.7111111),
                    offset: None,
                    units:  Some("degrees"),
                }))
            },
            73 => {
                Ok(Record::EnhancedSpeed(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  Some(1000.0),
                    offset: None,
                    units:  Some("m/s"),
                }))
            },
            78 => {
                Ok(Record::EnhancedAltitude(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  Some(5.0),
                    offset: Some(500.0),
                    units:  Some("m"),
                }))
            },
            81 => {
                Ok(Record::BatterySoc(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  Some(2.0),
                    offset: None,
                    units:  Some("percent"),
                }))
            },
            82 => {
                Ok(Record::MotorPower(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("watts"),
                }))
            },
            83 => {
                Ok(Record::VerticalRatio(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  Some(100.0),
                    offset: None,
                    units:  Some("percent"),
                }))
            },
            84 => {
                Ok(Record::StanceTimeBalance(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  Some(100.0),
                    offset: None,
                    units:  Some("percent"),
                }))
            },
            85 => {
                Ok(Record::StepLength(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  Some(10.0),
                    offset: None,
                    units:  Some("mm"),
                }))
            },
            91 => {
                Ok(Record::AbsolutePressure(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("Pa"),
                }))
            },
            92 => {
                Ok(Record::Depth(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  Some(1000.0),
                    offset: None,
                    units:  Some("m"),
                }))
            },
            93 => {
                Ok(Record::NextStopDepth(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  Some(1000.0),
                    offset: None,
                    units:  Some("m"),
                }))
            },
            94 => {
                Ok(Record::NextStopTime(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  Some(1.0),
                    offset: None,
                    units:  Some("s"),
                }))
            },
            95 => {
                Ok(Record::TimeToSurface(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  Some(1.0),
                    offset: None,
                    units:  Some("s"),
                }))
            },
            96 => {
                Ok(Record::NdlTime(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  Some(1.0),
                    offset: None,
                    units:  Some("s"),
                }))
            },
            97 => {
                Ok(Record::CnsLoad(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("percent"),
                }))
            },
            98 => {
                Ok(Record::N2Load(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  Some(1.0),
                    offset: None,
                    units:  Some("percent"),
                }))
            },
            _ => {
                Ok(Record::Unknown {
                    data: buffer.to_vec(),
                    field_def_num,
                })
            },
        }
    }
}
#[derive(Debug,Clone)]
pub enum Event {
    Timestamp(Field<profile::types::DateTime>),
    Event(Field<profile::types::Event>),
    EventType(Field<profile::types::EventType>),
    Data16(Field<profile::base::Uint16>),
    Data(Field<profile::base::Uint32>),
    EventGroup(Field<profile::base::Uint8>),
    #[doc = "Do not populate directly.  Autogenerated by decoder for \
             sport_point subfield components"]
    Score(Field<profile::base::Uint16>),
    #[doc = "Do not populate directly.  Autogenerated by decoder for \
             sport_point subfield components"]
    OpponentScore(Field<profile::base::Uint16>),
    #[doc = "Do not populate directly.  Autogenerated by decoder for \
             gear_change subfield components.  Front gear number. 1 is \
             innermost."]
    FrontGearNum(Field<profile::base::Uint8z>),
    #[doc = "Do not populate directly.  Autogenerated by decoder for \
             gear_change subfield components.  Number of front teeth."]
    FrontGear(Field<profile::base::Uint8z>),
    #[doc = "Do not populate directly.  Autogenerated by decoder for \
             gear_change subfield components.  Rear gear number. 1 is \
             innermost."]
    RearGearNum(Field<profile::base::Uint8z>),
    #[doc = "Do not populate directly.  Autogenerated by decoder for \
             gear_change subfield components.  Number of rear teeth."]
    RearGear(Field<profile::base::Uint8z>),
    DeviceIndex(Field<profile::types::DeviceIndex>),
    Unknown {
        data:          Vec<u8>,
        field_def_num: u8,
    },
}
impl Event {
    pub(crate) fn decode<T: ByteOrder>(
        buffer: &[u8],
        field_def_num: u8,
    ) -> error::Result<Self> {
        match field_def_num {
            253 => {
                Ok(Event::Timestamp(Field {
                    raw_value:  profile::types::DateTime::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("s"),
                }))
            },
            0 => {
                Ok(Event::Event(Field {
                    raw_value:  profile::types::Event::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            1 => {
                Ok(Event::EventType(Field {
                    raw_value:  profile::types::EventType::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            2 => {
                Ok(Event::Data16(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            3 => {
                Ok(Event::Data(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            4 => {
                Ok(Event::EventGroup(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            7 => {
                Ok(Event::Score(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            8 => {
                Ok(Event::OpponentScore(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            9 => {
                Ok(Event::FrontGearNum(Field {
                    raw_value:  profile::base::Uint8z::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            10 => {
                Ok(Event::FrontGear(Field {
                    raw_value:  profile::base::Uint8z::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            11 => {
                Ok(Event::RearGearNum(Field {
                    raw_value:  profile::base::Uint8z::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            12 => {
                Ok(Event::RearGear(Field {
                    raw_value:  profile::base::Uint8z::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            13 => {
                Ok(Event::DeviceIndex(Field {
                    raw_value:  profile::types::DeviceIndex::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            _ => {
                Ok(Event::Unknown {
                    data: buffer.to_vec(),
                    field_def_num,
                })
            },
        }
    }
}
#[derive(Debug,Clone)]
pub enum DeviceInfo {
    Timestamp(Field<profile::types::DateTime>),
    DeviceIndex(Field<profile::types::DeviceIndex>),
    DeviceType(Field<profile::base::Uint8>),
    Manufacturer(Field<profile::types::Manufacturer>),
    SerialNumber(Field<profile::base::Uint32z>),
    Product(Field<profile::base::Uint16>),
    SoftwareVersion(Field<profile::base::Uint16>),
    HardwareVersion(Field<profile::base::Uint8>),
    #[doc = "Reset by new battery or charge."]
    CumOperatingTime(Field<profile::base::Uint32>),
    BatteryVoltage(Field<profile::base::Uint16>),
    BatteryStatus(Field<profile::types::BatteryStatus>),
    #[doc = "Indicates the location of the sensor"]
    SensorPosition(Field<profile::types::BodyLocation>),
    #[doc = "Used to describe the sensor or location"]
    Descriptor(Field<profile::base::Utf8String>),
    AntTransmissionType(Field<profile::base::Uint8z>),
    AntDeviceNumber(Field<profile::base::Uint16z>),
    AntNetwork(Field<profile::types::AntNetwork>),
    SourceType(Field<profile::types::SourceType>),
    #[doc = "Optional free form string to indicate the devices name or model"]
    ProductName(Field<profile::base::Utf8String>),
    Unknown {
        data:          Vec<u8>,
        field_def_num: u8,
    },
}
impl DeviceInfo {
    pub(crate) fn decode<T: ByteOrder>(
        buffer: &[u8],
        field_def_num: u8,
    ) -> error::Result<Self> {
        match field_def_num {
            253 => {
                Ok(DeviceInfo::Timestamp(Field {
                    raw_value:  profile::types::DateTime::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("s"),
                }))
            },
            0 => {
                Ok(DeviceInfo::DeviceIndex(Field {
                    raw_value:  profile::types::DeviceIndex::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            1 => {
                Ok(DeviceInfo::DeviceType(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            2 => {
                Ok(DeviceInfo::Manufacturer(Field {
                    raw_value:  profile::types::Manufacturer::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            3 => {
                Ok(DeviceInfo::SerialNumber(Field {
                    raw_value:  profile::base::Uint32z::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            4 => {
                Ok(DeviceInfo::Product(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            5 => {
                Ok(DeviceInfo::SoftwareVersion(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  Some(100.0),
                    offset: None,
                    units:  None,
                }))
            },
            6 => {
                Ok(DeviceInfo::HardwareVersion(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            7 => {
                Ok(DeviceInfo::CumOperatingTime(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("s"),
                }))
            },
            10 => {
                Ok(DeviceInfo::BatteryVoltage(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  Some(256.0),
                    offset: None,
                    units:  Some("V"),
                }))
            },
            11 => {
                Ok(DeviceInfo::BatteryStatus(Field {
                    raw_value:  profile::types::BatteryStatus::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            18 => {
                Ok(DeviceInfo::SensorPosition(Field {
                    raw_value:  profile::types::BodyLocation::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            19 => {
                Ok(DeviceInfo::Descriptor(Field {
                    raw_value:  profile::base::Utf8String::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            20 => {
                Ok(DeviceInfo::AntTransmissionType(Field {
                    raw_value:  profile::base::Uint8z::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            21 => {
                Ok(DeviceInfo::AntDeviceNumber(Field {
                    raw_value:  profile::base::Uint16z::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            22 => {
                Ok(DeviceInfo::AntNetwork(Field {
                    raw_value:  profile::types::AntNetwork::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            25 => {
                Ok(DeviceInfo::SourceType(Field {
                    raw_value:  profile::types::SourceType::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            27 => {
                Ok(DeviceInfo::ProductName(Field {
                    raw_value:  profile::base::Utf8String::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            _ => {
                Ok(DeviceInfo::Unknown {
                    data: buffer.to_vec(),
                    field_def_num,
                })
            },
        }
    }
}
#[doc = "Corresponds to file_id of workout or course."]
#[derive(Debug,Clone)]
pub enum TrainingFile {
    Timestamp(Field<profile::types::DateTime>),
    Type(Field<profile::types::File>),
    Manufacturer(Field<profile::types::Manufacturer>),
    Product(Field<profile::base::Uint16>),
    SerialNumber(Field<profile::base::Uint32z>),
    TimeCreated(Field<profile::types::DateTime>),
    Unknown { data:          Vec<u8>, field_def_num: u8 },
}
impl TrainingFile {
    pub(crate) fn decode<T: ByteOrder>(
        buffer: &[u8],
        field_def_num: u8,
    ) -> error::Result<Self> {
        match field_def_num {
            253 => {
                Ok(TrainingFile::Timestamp(Field {
                    raw_value:  profile::types::DateTime::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            0 => {
                Ok(TrainingFile::Type(Field {
                    raw_value:  profile::types::File::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            1 => {
                Ok(TrainingFile::Manufacturer(Field {
                    raw_value:  profile::types::Manufacturer::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            2 => {
                Ok(TrainingFile::Product(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            3 => {
                Ok(TrainingFile::SerialNumber(Field {
                    raw_value:  profile::base::Uint32z::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            4 => {
                Ok(TrainingFile::TimeCreated(Field {
                    raw_value:  profile::types::DateTime::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            _ => {
                Ok(TrainingFile::Unknown {
                    data: buffer.to_vec(),
                    field_def_num,
                })
            },
        }
    }
}
#[doc = "Heart rate variability"]
#[derive(Debug,Clone)]
pub enum Hrv {
    #[doc = "Time between beats"]
    Time(Field<profile::base::Uint16>),
    Unknown {
        data:          Vec<u8>,
        field_def_num: u8,
    },
}
impl Hrv {
    pub(crate) fn decode<T: ByteOrder>(
        buffer: &[u8],
        field_def_num: u8,
    ) -> error::Result<Self> {
        match field_def_num {
            0 => {
                Ok(Hrv::Time(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  Some(1000.0),
                    offset: None,
                    units:  Some("s"),
                }))
            },
            _ => {
                Ok(Hrv::Unknown {
                    data: buffer.to_vec(),
                    field_def_num,
                })
            },
        }
    }
}
#[derive(Debug,Clone)]
pub enum WeatherConditions {
    #[doc = "time of update for current conditions, else forecast time"]
    Timestamp(Field<profile::types::DateTime>),
    #[doc = "Current or forecast"]
    WeatherReport(Field<profile::types::WeatherReport>),
    Temperature(Field<profile::base::Sint8>),
    #[doc = "Corresponds to GSC Response weatherIcon field"]
    Condition(Field<profile::types::WeatherStatus>),
    WindDirection(Field<profile::base::Uint16>),
    WindSpeed(Field<profile::base::Uint16>),
    #[doc = "range 0-100"]
    PrecipitationProbability(Field<profile::base::Uint8>),
    #[doc = "Heat Index if  GCS heatIdx above or equal to 90F or wind chill \
             if GCS windChill below or equal to 32F"]
    TemperatureFeelsLike(Field<profile::base::Sint8>),
    RelativeHumidity(Field<profile::base::Uint8>),
    #[doc = "string corresponding to GCS response location string"]
    Location(Field<profile::base::Utf8String>),
    ObservedAtTime(Field<profile::types::DateTime>),
    ObservedLocationLat(Field<profile::base::Sint32>),
    ObservedLocationLong(Field<profile::base::Sint32>),
    DayOfWeek(Field<profile::types::DayOfWeek>),
    HighTemperature(Field<profile::base::Sint8>),
    LowTemperature(Field<profile::base::Sint8>),
    Unknown {
        data:          Vec<u8>,
        field_def_num: u8,
    },
}
impl WeatherConditions {
    pub(crate) fn decode<T: ByteOrder>(
        buffer: &[u8],
        field_def_num: u8,
    ) -> error::Result<Self> {
        match field_def_num {
            253 => {
                Ok(WeatherConditions::Timestamp(Field {
                    raw_value:  profile::types::DateTime::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            0 => {
                Ok(WeatherConditions::WeatherReport(Field {
                    raw_value:  profile::types::WeatherReport::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            1 => {
                Ok(WeatherConditions::Temperature(Field {
                    raw_value:  profile::base::Sint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("C"),
                }))
            },
            2 => {
                Ok(WeatherConditions::Condition(Field {
                    raw_value:  profile::types::WeatherStatus::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            3 => {
                Ok(WeatherConditions::WindDirection(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("degrees"),
                }))
            },
            4 => {
                Ok(WeatherConditions::WindSpeed(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  Some(1000.0),
                    offset: None,
                    units:  Some("m/s"),
                }))
            },
            5 => {
                Ok(WeatherConditions::PrecipitationProbability(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            6 => {
                Ok(WeatherConditions::TemperatureFeelsLike(Field {
                    raw_value:  profile::base::Sint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("C"),
                }))
            },
            7 => {
                Ok(WeatherConditions::RelativeHumidity(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            8 => {
                Ok(WeatherConditions::Location(Field {
                    raw_value:  profile::base::Utf8String::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            9 => {
                Ok(WeatherConditions::ObservedAtTime(Field {
                    raw_value:  profile::types::DateTime::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            10 => {
                Ok(WeatherConditions::ObservedLocationLat(Field {
                    raw_value:  profile::base::Sint32::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("semicircles"),
                }))
            },
            11 => {
                Ok(WeatherConditions::ObservedLocationLong(Field {
                    raw_value:  profile::base::Sint32::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("semicircles"),
                }))
            },
            12 => {
                Ok(WeatherConditions::DayOfWeek(Field {
                    raw_value:  profile::types::DayOfWeek::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            13 => {
                Ok(WeatherConditions::HighTemperature(Field {
                    raw_value:  profile::base::Sint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("C"),
                }))
            },
            14 => {
                Ok(WeatherConditions::LowTemperature(Field {
                    raw_value:  profile::base::Sint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("C"),
                }))
            },
            _ => {
                Ok(WeatherConditions::Unknown {
                    data: buffer.to_vec(),
                    field_def_num,
                })
            },
        }
    }
}
#[derive(Debug,Clone)]
pub enum WeatherAlert {
    Timestamp(Field<profile::types::DateTime>),
    #[doc = "Unique identifier from GCS report ID string, length is 12"]
    ReportId(Field<profile::base::Utf8String>),
    #[doc = "Time alert was issued"]
    IssueTime(Field<profile::types::DateTime>),
    #[doc = "Time alert expires"]
    ExpireTime(Field<profile::types::DateTime>),
    #[doc = "Warning, Watch, Advisory, Statement"]
    Severity(Field<profile::types::WeatherSeverity>),
    #[doc = "Tornado, Severe Thunderstorm, etc."]
    Type(Field<profile::types::WeatherSevereType>),
    Unknown {
        data:          Vec<u8>,
        field_def_num: u8,
    },
}
impl WeatherAlert {
    pub(crate) fn decode<T: ByteOrder>(
        buffer: &[u8],
        field_def_num: u8,
    ) -> error::Result<Self> {
        match field_def_num {
            253 => {
                Ok(WeatherAlert::Timestamp(Field {
                    raw_value:  profile::types::DateTime::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            0 => {
                Ok(WeatherAlert::ReportId(Field {
                    raw_value:  profile::base::Utf8String::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            1 => {
                Ok(WeatherAlert::IssueTime(Field {
                    raw_value:  profile::types::DateTime::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            2 => {
                Ok(WeatherAlert::ExpireTime(Field {
                    raw_value:  profile::types::DateTime::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            3 => {
                Ok(WeatherAlert::Severity(Field {
                    raw_value:  profile::types::WeatherSeverity::decode::<T>(
                        buffer,
                    )?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            4 => {
                Ok(WeatherAlert::Type(Field {
                    raw_value:  profile::types::WeatherSevereType::decode::<T>(
                        buffer,
                    )?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            _ => {
                Ok(WeatherAlert::Unknown {
                    data: buffer.to_vec(),
                    field_def_num,
                })
            },
        }
    }
}
#[derive(Debug,Clone)]
pub enum GpsMetadata {
    #[doc = "Whole second part of the timestamp."]
    Timestamp(Field<profile::types::DateTime>),
    #[doc = "Millisecond part of the timestamp."]
    TimestampMs(Field<profile::base::Uint16>),
    PositionLat(Field<profile::base::Sint32>),
    PositionLong(Field<profile::base::Sint32>),
    EnhancedAltitude(Field<profile::base::Uint32>),
    EnhancedSpeed(Field<profile::base::Uint32>),
    Heading(Field<profile::base::Uint16>),
    #[doc = "Used to correlate UTC to system time if the timestamp of the \
             message is in system time.  This UTC time is derived from the \
             GPS data."]
    UtcTimestamp(Field<profile::types::DateTime>),
    #[doc = "velocity\\[0\\] is lon velocity.  Velocity\\[1\\] is lat \
             velocity.  Velocity\\[2\\] is altitude velocity."]
    Velocity(Field<profile::base::Sint16>),
    Unknown {
        data:          Vec<u8>,
        field_def_num: u8,
    },
}
impl GpsMetadata {
    pub(crate) fn decode<T: ByteOrder>(
        buffer: &[u8],
        field_def_num: u8,
    ) -> error::Result<Self> {
        match field_def_num {
            253 => {
                Ok(GpsMetadata::Timestamp(Field {
                    raw_value:  profile::types::DateTime::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("s"),
                }))
            },
            0 => {
                Ok(GpsMetadata::TimestampMs(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("ms"),
                }))
            },
            1 => {
                Ok(GpsMetadata::PositionLat(Field {
                    raw_value:  profile::base::Sint32::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("semicircles"),
                }))
            },
            2 => {
                Ok(GpsMetadata::PositionLong(Field {
                    raw_value:  profile::base::Sint32::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("semicircles"),
                }))
            },
            3 => {
                Ok(GpsMetadata::EnhancedAltitude(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  Some(5.0),
                    offset: Some(500.0),
                    units:  Some("m"),
                }))
            },
            4 => {
                Ok(GpsMetadata::EnhancedSpeed(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  Some(1000.0),
                    offset: None,
                    units:  Some("m/s"),
                }))
            },
            5 => {
                Ok(GpsMetadata::Heading(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  Some(100.0),
                    offset: None,
                    units:  Some("degrees"),
                }))
            },
            6 => {
                Ok(GpsMetadata::UtcTimestamp(Field {
                    raw_value:  profile::types::DateTime::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("s"),
                }))
            },
            7 => {
                Ok(GpsMetadata::Velocity(Field {
                    raw_value:  profile::base::Sint16::decode::<T>(buffer)?,
                    scale:  Some(100.0),
                    offset: None,
                    units:  Some("m/s"),
                }))
            },
            _ => {
                Ok(GpsMetadata::Unknown {
                    data: buffer.to_vec(),
                    field_def_num,
                })
            },
        }
    }
}
#[derive(Debug,Clone)]
pub enum CameraEvent {
    #[doc = "Whole second part of the timestamp."]
    Timestamp(Field<profile::types::DateTime>),
    #[doc = "Millisecond part of the timestamp."]
    TimestampMs(Field<profile::base::Uint16>),
    CameraEventType(Field<profile::types::CameraEventType>),
    CameraFileUuid(Field<profile::base::Utf8String>),
    CameraOrientation(Field<profile::types::CameraOrientationType>),
    Unknown {
        data:          Vec<u8>,
        field_def_num: u8,
    },
}
impl CameraEvent {
    pub(crate) fn decode<T: ByteOrder>(
        buffer: &[u8],
        field_def_num: u8,
    ) -> error::Result<Self> {
        match field_def_num {
            253 => {
                Ok(CameraEvent::Timestamp(Field {
                    raw_value:  profile::types::DateTime::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("s"),
                }))
            },
            0 => {
                Ok(CameraEvent::TimestampMs(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("ms"),
                }))
            },
            1 => {
                Ok(CameraEvent::CameraEventType(Field {
                    raw_value:  profile::types::CameraEventType::decode::<T>(
                        buffer,
                    )?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            2 => {
                Ok(CameraEvent::CameraFileUuid(Field {
                    raw_value:  profile::base::Utf8String::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            3 => {
                Ok(CameraEvent::CameraOrientation(Field {
                    raw_value:  profile::types::CameraOrientationType::decode::<T>(
                        buffer,
                    )?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            _ => {
                Ok(CameraEvent::Unknown {
                    data: buffer.to_vec(),
                    field_def_num,
                })
            },
        }
    }
}
#[derive(Debug,Clone)]
pub enum GyroscopeData {
    #[doc = "Whole second part of the timestamp"]
    Timestamp(Field<profile::types::DateTime>),
    #[doc = "Millisecond part of the timestamp."]
    TimestampMs(Field<profile::base::Uint16>),
    #[doc = "Each time in the array describes the time at which the gyro \
             sample with the corrosponding index was taken. Limited to 30 \
             samples in each message. The samples may span across seconds. \
             Array size must match the number of samples in gyro_x and gyro_y \
             and gyro_z"]
    SampleTimeOffset(Field<profile::base::Uint16>),
    #[doc = "These are the raw ADC reading. Maximum number of samples is 30 \
             in each message. The samples may span across seconds. A \
             conversion will need to be done on this data once read."]
    GyroX(Field<profile::base::Uint16>),
    #[doc = "These are the raw ADC reading. Maximum number of samples is 30 \
             in each message. The samples may span across seconds. A \
             conversion will need to be done on this data once read."]
    GyroY(Field<profile::base::Uint16>),
    #[doc = "These are the raw ADC reading. Maximum number of samples is 30 \
             in each message. The samples may span across seconds. A \
             conversion will need to be done on this data once read."]
    GyroZ(Field<profile::base::Uint16>),
    #[doc = "Calibrated gyro reading"]
    CalibratedGyroX(Field<profile::base::Float32>),
    #[doc = "Calibrated gyro reading"]
    CalibratedGyroY(Field<profile::base::Float32>),
    #[doc = "Calibrated gyro reading"]
    CalibratedGyroZ(Field<profile::base::Float32>),
    Unknown {
        data:          Vec<u8>,
        field_def_num: u8,
    },
}
impl GyroscopeData {
    pub(crate) fn decode<T: ByteOrder>(
        buffer: &[u8],
        field_def_num: u8,
    ) -> error::Result<Self> {
        match field_def_num {
            253 => {
                Ok(GyroscopeData::Timestamp(Field {
                    raw_value:  profile::types::DateTime::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("s"),
                }))
            },
            0 => {
                Ok(GyroscopeData::TimestampMs(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("ms"),
                }))
            },
            1 => {
                Ok(GyroscopeData::SampleTimeOffset(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("ms"),
                }))
            },
            2 => {
                Ok(GyroscopeData::GyroX(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("counts"),
                }))
            },
            3 => {
                Ok(GyroscopeData::GyroY(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("counts"),
                }))
            },
            4 => {
                Ok(GyroscopeData::GyroZ(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("counts"),
                }))
            },
            5 => {
                Ok(GyroscopeData::CalibratedGyroX(Field {
                    raw_value:  profile::base::Float32::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("deg/s"),
                }))
            },
            6 => {
                Ok(GyroscopeData::CalibratedGyroY(Field {
                    raw_value:  profile::base::Float32::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("deg/s"),
                }))
            },
            7 => {
                Ok(GyroscopeData::CalibratedGyroZ(Field {
                    raw_value:  profile::base::Float32::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("deg/s"),
                }))
            },
            _ => {
                Ok(GyroscopeData::Unknown {
                    data: buffer.to_vec(),
                    field_def_num,
                })
            },
        }
    }
}
#[derive(Debug,Clone)]
pub enum AccelerometerData {
    #[doc = "Whole second part of the timestamp"]
    Timestamp(Field<profile::types::DateTime>),
    #[doc = "Millisecond part of the timestamp."]
    TimestampMs(Field<profile::base::Uint16>),
    #[doc = "Each time in the array describes the time at which the \
             accelerometer sample with the corrosponding index was taken. \
             Limited to 30 samples in each message. The samples may span \
             across seconds. Array size must match the number of samples in \
             accel_x and accel_y and accel_z"]
    SampleTimeOffset(Field<profile::base::Uint16>),
    #[doc = "These are the raw ADC reading. Maximum number of samples is 30 \
             in each message. The samples may span across seconds. A \
             conversion will need to be done on this data once read."]
    AccelX(Field<profile::base::Uint16>),
    #[doc = "These are the raw ADC reading. Maximum number of samples is 30 \
             in each message. The samples may span across seconds. A \
             conversion will need to be done on this data once read."]
    AccelY(Field<profile::base::Uint16>),
    #[doc = "These are the raw ADC reading. Maximum number of samples is 30 \
             in each message. The samples may span across seconds. A \
             conversion will need to be done on this data once read."]
    AccelZ(Field<profile::base::Uint16>),
    #[doc = "Calibrated accel reading"]
    CalibratedAccelX(Field<profile::base::Float32>),
    #[doc = "Calibrated accel reading"]
    CalibratedAccelY(Field<profile::base::Float32>),
    #[doc = "Calibrated accel reading"]
    CalibratedAccelZ(Field<profile::base::Float32>),
    #[doc = "Calibrated accel reading"]
    CompressedCalibratedAccelX(Field<profile::base::Sint16>),
    #[doc = "Calibrated accel reading"]
    CompressedCalibratedAccelY(Field<profile::base::Sint16>),
    #[doc = "Calibrated accel reading"]
    CompressedCalibratedAccelZ(Field<profile::base::Sint16>),
    Unknown {
        data:          Vec<u8>,
        field_def_num: u8,
    },
}
impl AccelerometerData {
    pub(crate) fn decode<T: ByteOrder>(
        buffer: &[u8],
        field_def_num: u8,
    ) -> error::Result<Self> {
        match field_def_num {
            253 => {
                Ok(AccelerometerData::Timestamp(Field {
                    raw_value:  profile::types::DateTime::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("s"),
                }))
            },
            0 => {
                Ok(AccelerometerData::TimestampMs(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("ms"),
                }))
            },
            1 => {
                Ok(AccelerometerData::SampleTimeOffset(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("ms"),
                }))
            },
            2 => {
                Ok(AccelerometerData::AccelX(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("counts"),
                }))
            },
            3 => {
                Ok(AccelerometerData::AccelY(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("counts"),
                }))
            },
            4 => {
                Ok(AccelerometerData::AccelZ(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("counts"),
                }))
            },
            5 => {
                Ok(AccelerometerData::CalibratedAccelX(Field {
                    raw_value:  profile::base::Float32::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("g"),
                }))
            },
            6 => {
                Ok(AccelerometerData::CalibratedAccelY(Field {
                    raw_value:  profile::base::Float32::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("g"),
                }))
            },
            7 => {
                Ok(AccelerometerData::CalibratedAccelZ(Field {
                    raw_value:  profile::base::Float32::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("g"),
                }))
            },
            8 => {
                Ok(AccelerometerData::CompressedCalibratedAccelX(Field {
                    raw_value:  profile::base::Sint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("mG"),
                }))
            },
            9 => {
                Ok(AccelerometerData::CompressedCalibratedAccelY(Field {
                    raw_value:  profile::base::Sint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("mG"),
                }))
            },
            10 => {
                Ok(AccelerometerData::CompressedCalibratedAccelZ(Field {
                    raw_value:  profile::base::Sint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("mG"),
                }))
            },
            _ => {
                Ok(AccelerometerData::Unknown {
                    data: buffer.to_vec(),
                    field_def_num,
                })
            },
        }
    }
}
#[derive(Debug,Clone)]
pub enum MagnetometerData {
    #[doc = "Whole second part of the timestamp"]
    Timestamp(Field<profile::types::DateTime>),
    #[doc = "Millisecond part of the timestamp."]
    TimestampMs(Field<profile::base::Uint16>),
    #[doc = "Each time in the array describes the time at which the compass \
             sample with the corrosponding index was taken. Limited to 30 \
             samples in each message. The samples may span across seconds. \
             Array size must match the number of samples in cmps_x and cmps_y \
             and cmps_z"]
    SampleTimeOffset(Field<profile::base::Uint16>),
    #[doc = "These are the raw ADC reading. Maximum number of samples is 30 \
             in each message. The samples may span across seconds. A \
             conversion will need to be done on this data once read."]
    MagX(Field<profile::base::Uint16>),
    #[doc = "These are the raw ADC reading. Maximum number of samples is 30 \
             in each message. The samples may span across seconds. A \
             conversion will need to be done on this data once read."]
    MagY(Field<profile::base::Uint16>),
    #[doc = "These are the raw ADC reading. Maximum number of samples is 30 \
             in each message. The samples may span across seconds. A \
             conversion will need to be done on this data once read."]
    MagZ(Field<profile::base::Uint16>),
    #[doc = "Calibrated Magnetometer reading"]
    CalibratedMagX(Field<profile::base::Float32>),
    #[doc = "Calibrated Magnetometer reading"]
    CalibratedMagY(Field<profile::base::Float32>),
    #[doc = "Calibrated Magnetometer reading"]
    CalibratedMagZ(Field<profile::base::Float32>),
    Unknown {
        data:          Vec<u8>,
        field_def_num: u8,
    },
}
impl MagnetometerData {
    pub(crate) fn decode<T: ByteOrder>(
        buffer: &[u8],
        field_def_num: u8,
    ) -> error::Result<Self> {
        match field_def_num {
            253 => {
                Ok(MagnetometerData::Timestamp(Field {
                    raw_value:  profile::types::DateTime::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("s"),
                }))
            },
            0 => {
                Ok(MagnetometerData::TimestampMs(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("ms"),
                }))
            },
            1 => {
                Ok(MagnetometerData::SampleTimeOffset(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("ms"),
                }))
            },
            2 => {
                Ok(MagnetometerData::MagX(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("counts"),
                }))
            },
            3 => {
                Ok(MagnetometerData::MagY(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("counts"),
                }))
            },
            4 => {
                Ok(MagnetometerData::MagZ(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("counts"),
                }))
            },
            5 => {
                Ok(MagnetometerData::CalibratedMagX(Field {
                    raw_value:  profile::base::Float32::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("G"),
                }))
            },
            6 => {
                Ok(MagnetometerData::CalibratedMagY(Field {
                    raw_value:  profile::base::Float32::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("G"),
                }))
            },
            7 => {
                Ok(MagnetometerData::CalibratedMagZ(Field {
                    raw_value:  profile::base::Float32::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("G"),
                }))
            },
            _ => {
                Ok(MagnetometerData::Unknown {
                    data: buffer.to_vec(),
                    field_def_num,
                })
            },
        }
    }
}
#[derive(Debug,Clone)]
pub enum BarometerData {
    #[doc = "Whole second part of the timestamp"]
    Timestamp(Field<profile::types::DateTime>),
    #[doc = "Millisecond part of the timestamp."]
    TimestampMs(Field<profile::base::Uint16>),
    #[doc = "Each time in the array describes the time at which the barometer \
             sample with the corrosponding index was taken. The samples may \
             span across seconds. Array size must match the number of samples \
             in baro_cal"]
    SampleTimeOffset(Field<profile::base::Uint16>),
    #[doc = "These are the raw ADC reading. The samples may span across \
             seconds. A conversion will need to be done on this data once \
             read."]
    BaroPres(Field<profile::base::Uint32>),
    Unknown {
        data:          Vec<u8>,
        field_def_num: u8,
    },
}
impl BarometerData {
    pub(crate) fn decode<T: ByteOrder>(
        buffer: &[u8],
        field_def_num: u8,
    ) -> error::Result<Self> {
        match field_def_num {
            253 => {
                Ok(BarometerData::Timestamp(Field {
                    raw_value:  profile::types::DateTime::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("s"),
                }))
            },
            0 => {
                Ok(BarometerData::TimestampMs(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("ms"),
                }))
            },
            1 => {
                Ok(BarometerData::SampleTimeOffset(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("ms"),
                }))
            },
            2 => {
                Ok(BarometerData::BaroPres(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("Pa"),
                }))
            },
            _ => {
                Ok(BarometerData::Unknown {
                    data: buffer.to_vec(),
                    field_def_num,
                })
            },
        }
    }
}
#[derive(Debug,Clone)]
pub enum ThreeDSensorCalibration {
    #[doc = "Whole second part of the timestamp"]
    Timestamp(Field<profile::types::DateTime>),
    #[doc = "Indicates which sensor the calibration is for"]
    SensorType(Field<profile::types::SensorType>),
    #[doc = "Calibration factor used to convert from raw ADC raw_value to \
             degrees, g,  etc."]
    CalibrationFactor(Field<profile::base::Uint32>),
    #[doc = "Calibration factor divisor"]
    CalibrationDivisor(Field<profile::base::Uint32>),
    #[doc = "Level shift raw_value used to shift the ADC raw_value back into range"]
    LevelShift(Field<profile::base::Uint32>),
    #[doc = "Internal calibration factors, one for each: xy, yx, zx"]
    OffsetCal(Field<profile::base::Sint32>),
    #[doc = "3 x 3 rotation matrix (row major)"]
    OrientationMatrix(Field<profile::base::Sint32>),
    Unknown {
        data:          Vec<u8>,
        field_def_num: u8,
    },
}
impl ThreeDSensorCalibration {
    pub(crate) fn decode<T: ByteOrder>(
        buffer: &[u8],
        field_def_num: u8,
    ) -> error::Result<Self> {
        match field_def_num {
            253 => {
                Ok(ThreeDSensorCalibration::Timestamp(Field {
                    raw_value:  profile::types::DateTime::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("s"),
                }))
            },
            0 => {
                Ok(ThreeDSensorCalibration::SensorType(Field {
                    raw_value:  profile::types::SensorType::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            1 => {
                Ok(ThreeDSensorCalibration::CalibrationFactor(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            2 => {
                Ok(ThreeDSensorCalibration::CalibrationDivisor(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("counts"),
                }))
            },
            3 => {
                Ok(ThreeDSensorCalibration::LevelShift(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            4 => {
                Ok(ThreeDSensorCalibration::OffsetCal(Field {
                    raw_value:  profile::base::Sint32::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            5 => {
                Ok(ThreeDSensorCalibration::OrientationMatrix(Field {
                    raw_value:  profile::base::Sint32::decode::<T>(buffer)?,
                    scale:  Some(65535.0),
                    offset: None,
                    units:  None,
                }))
            },
            _ => {
                Ok(ThreeDSensorCalibration::Unknown {
                    data: buffer.to_vec(),
                    field_def_num,
                })
            },
        }
    }
}
#[derive(Debug,Clone)]
pub enum OneDSensorCalibration {
    #[doc = "Whole second part of the timestamp"]
    Timestamp(Field<profile::types::DateTime>),
    #[doc = "Indicates which sensor the calibration is for"]
    SensorType(Field<profile::types::SensorType>),
    #[doc = "Calibration factor used to convert from raw ADC raw_value to \
             degrees, g,  etc."]
    CalibrationFactor(Field<profile::base::Uint32>),
    #[doc = "Calibration factor divisor"]
    CalibrationDivisor(Field<profile::base::Uint32>),
    #[doc = "Level shift raw_value used to shift the ADC raw_value back into range"]
    LevelShift(Field<profile::base::Uint32>),
    #[doc = "Internal Calibration factor"]
    OffsetCal(Field<profile::base::Sint32>),
    Unknown {
        data:          Vec<u8>,
        field_def_num: u8,
    },
}
impl OneDSensorCalibration {
    pub(crate) fn decode<T: ByteOrder>(
        buffer: &[u8],
        field_def_num: u8,
    ) -> error::Result<Self> {
        match field_def_num {
            253 => {
                Ok(OneDSensorCalibration::Timestamp(Field {
                    raw_value:  profile::types::DateTime::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("s"),
                }))
            },
            0 => {
                Ok(OneDSensorCalibration::SensorType(Field {
                    raw_value:  profile::types::SensorType::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            1 => {
                Ok(OneDSensorCalibration::CalibrationFactor(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            2 => {
                Ok(OneDSensorCalibration::CalibrationDivisor(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("counts"),
                }))
            },
            3 => {
                Ok(OneDSensorCalibration::LevelShift(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            4 => {
                Ok(OneDSensorCalibration::OffsetCal(Field {
                    raw_value:  profile::base::Sint32::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            _ => {
                Ok(OneDSensorCalibration::Unknown {
                    data: buffer.to_vec(),
                    field_def_num,
                })
            },
        }
    }
}
#[derive(Debug,Clone)]
pub enum VideoFrame {
    #[doc = "Whole second part of the timestamp"]
    Timestamp(Field<profile::types::DateTime>),
    #[doc = "Millisecond part of the timestamp."]
    TimestampMs(Field<profile::base::Uint16>),
    #[doc = "Number of the frame that the timestamp and timestamp_ms \
             correlate to"]
    FrameNumber(Field<profile::base::Uint32>),
    Unknown {
        data:          Vec<u8>,
        field_def_num: u8,
    },
}
impl VideoFrame {
    pub(crate) fn decode<T: ByteOrder>(
        buffer: &[u8],
        field_def_num: u8,
    ) -> error::Result<Self> {
        match field_def_num {
            253 => {
                Ok(VideoFrame::Timestamp(Field {
                    raw_value:  profile::types::DateTime::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("s"),
                }))
            },
            0 => {
                Ok(VideoFrame::TimestampMs(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("ms"),
                }))
            },
            1 => {
                Ok(VideoFrame::FrameNumber(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            _ => {
                Ok(VideoFrame::Unknown {
                    data: buffer.to_vec(),
                    field_def_num,
                })
            },
        }
    }
}
#[derive(Debug,Clone)]
pub enum ObdiiData {
    #[doc = "Timestamp message was output"]
    Timestamp(Field<profile::types::DateTime>),
    #[doc = "Fractional part of timestamp, added to timestamp"]
    TimestampMs(Field<profile::base::Uint16>),
    #[doc = "Offset of PID reading \\[i\\] from \
             start_timestamp+start_timestamp_ms. Readings may span accross \
             seconds."]
    TimeOffset(Field<profile::base::Uint16>),
    #[doc = "Parameter ID"]
    Pid(Field<profile::base::Bytes>),
    #[doc = "Raw parameter data"]
    RawData(Field<profile::base::Bytes>),
    #[doc = "Optional, data size of PID\\[i\\].  If not specified refer to \
             SAE J1979."]
    PidDataSize(Field<profile::base::Uint8>),
    #[doc = "System time associated with sample expressed in ms, can be used \
             instead of time_offset.  There will be a system_time raw_value for \
             each raw_data element.  For multibyte pids the system_time is \
             repeated."]
    SystemTime(Field<profile::base::Uint32>),
    #[doc = "Timestamp of first sample recorded in the message.  Used with \
             time_offset to generate time of each sample"]
    StartTimestamp(Field<profile::types::DateTime>),
    #[doc = "Fractional part of start_timestamp"]
    StartTimestampMs(Field<profile::base::Uint16>),
    Unknown {
        data:          Vec<u8>,
        field_def_num: u8,
    },
}
impl ObdiiData {
    pub(crate) fn decode<T: ByteOrder>(
        buffer: &[u8],
        field_def_num: u8,
    ) -> error::Result<Self> {
        match field_def_num {
            253 => {
                Ok(ObdiiData::Timestamp(Field {
                    raw_value:  profile::types::DateTime::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("s"),
                }))
            },
            0 => {
                Ok(ObdiiData::TimestampMs(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("ms"),
                }))
            },
            1 => {
                Ok(ObdiiData::TimeOffset(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("ms"),
                }))
            },
            2 => {
                Ok(ObdiiData::Pid(Field {
                    raw_value:  profile::base::Bytes::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            3 => {
                Ok(ObdiiData::RawData(Field {
                    raw_value:  profile::base::Bytes::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            4 => {
                Ok(ObdiiData::PidDataSize(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            5 => {
                Ok(ObdiiData::SystemTime(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            6 => {
                Ok(ObdiiData::StartTimestamp(Field {
                    raw_value:  profile::types::DateTime::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            7 => {
                Ok(ObdiiData::StartTimestampMs(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("ms"),
                }))
            },
            _ => {
                Ok(ObdiiData::Unknown {
                    data: buffer.to_vec(),
                    field_def_num,
                })
            },
        }
    }
}
#[derive(Debug,Clone)]
pub enum NmeaSentence {
    #[doc = "Timestamp message was output"]
    Timestamp(Field<profile::types::DateTime>),
    #[doc = "Fractional part of timestamp, added to timestamp"]
    TimestampMs(Field<profile::base::Uint16>),
    #[doc = "NMEA sentence"]
    Sentence(Field<profile::base::Utf8String>),
    Unknown {
        data:          Vec<u8>,
        field_def_num: u8,
    },
}
impl NmeaSentence {
    pub(crate) fn decode<T: ByteOrder>(
        buffer: &[u8],
        field_def_num: u8,
    ) -> error::Result<Self> {
        match field_def_num {
            253 => {
                Ok(NmeaSentence::Timestamp(Field {
                    raw_value:  profile::types::DateTime::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("s"),
                }))
            },
            0 => {
                Ok(NmeaSentence::TimestampMs(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("ms"),
                }))
            },
            1 => {
                Ok(NmeaSentence::Sentence(Field {
                    raw_value:  profile::base::Utf8String::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            _ => {
                Ok(NmeaSentence::Unknown {
                    data: buffer.to_vec(),
                    field_def_num,
                })
            },
        }
    }
}
#[derive(Debug,Clone)]
pub enum AviationAttitude {
    #[doc = "Timestamp message was output"]
    Timestamp(Field<profile::types::DateTime>),
    #[doc = "Fractional part of timestamp, added to timestamp"]
    TimestampMs(Field<profile::base::Uint16>),
    #[doc = "System time associated with sample expressed in ms."]
    SystemTime(Field<profile::base::Uint32>),
    #[doc = "Range -PI/2 to +PI/2"]
    Pitch(Field<profile::base::Sint16>),
    #[doc = "Range -PI to +PI"]
    Roll(Field<profile::base::Sint16>),
    #[doc = "Range -78.4 to +78.4 (-8 Gs to 8 Gs)"]
    AccelLateral(Field<profile::base::Sint16>),
    #[doc = "Range -78.4 to +78.4 (-8 Gs to 8 Gs)"]
    AccelNormal(Field<profile::base::Sint16>),
    #[doc = "Range -8.727 to +8.727 (-500 degs/sec to +500 degs/sec)"]
    TurnRate(Field<profile::base::Sint16>),
    Stage(Field<profile::types::AttitudeStage>),
    #[doc = "The percent complete of the current attitude stage.  Set to 0 \
             for attitude stages 0, 1 and 2 and to 100 for attitude stage 3 \
             by AHRS modules that do not support it.  Range - 100"]
    AttitudeStageComplete(Field<profile::base::Uint8>),
    #[doc = "Track Angle/Heading Range 0 - 2pi"]
    Track(Field<profile::base::Uint16>),
    Validity(Field<profile::types::AttitudeValidity>),
    Unknown {
        data:          Vec<u8>,
        field_def_num: u8,
    },
}
impl AviationAttitude {
    pub(crate) fn decode<T: ByteOrder>(
        buffer: &[u8],
        field_def_num: u8,
    ) -> error::Result<Self> {
        match field_def_num {
            253 => {
                Ok(AviationAttitude::Timestamp(Field {
                    raw_value:  profile::types::DateTime::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("s"),
                }))
            },
            0 => {
                Ok(AviationAttitude::TimestampMs(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("ms"),
                }))
            },
            1 => {
                Ok(AviationAttitude::SystemTime(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("ms"),
                }))
            },
            2 => {
                Ok(AviationAttitude::Pitch(Field {
                    raw_value:  profile::base::Sint16::decode::<T>(buffer)?,
                    scale:  Some(10430.38),
                    offset: None,
                    units:  Some("radians"),
                }))
            },
            3 => {
                Ok(AviationAttitude::Roll(Field {
                    raw_value:  profile::base::Sint16::decode::<T>(buffer)?,
                    scale:  Some(10430.38),
                    offset: None,
                    units:  Some("radians"),
                }))
            },
            4 => {
                Ok(AviationAttitude::AccelLateral(Field {
                    raw_value:  profile::base::Sint16::decode::<T>(buffer)?,
                    scale:  Some(100.0),
                    offset: None,
                    units:  Some("m/s^2"),
                }))
            },
            5 => {
                Ok(AviationAttitude::AccelNormal(Field {
                    raw_value:  profile::base::Sint16::decode::<T>(buffer)?,
                    scale:  Some(100.0),
                    offset: None,
                    units:  Some("m/s^2"),
                }))
            },
            6 => {
                Ok(AviationAttitude::TurnRate(Field {
                    raw_value:  profile::base::Sint16::decode::<T>(buffer)?,
                    scale:  Some(1024.0),
                    offset: None,
                    units:  Some("radians/second"),
                }))
            },
            7 => {
                Ok(AviationAttitude::Stage(Field {
                    raw_value:  profile::types::AttitudeStage::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            8 => {
                Ok(AviationAttitude::AttitudeStageComplete(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("%"),
                }))
            },
            9 => {
                Ok(AviationAttitude::Track(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  Some(10430.38),
                    offset: None,
                    units:  Some("radians"),
                }))
            },
            10 => {
                Ok(AviationAttitude::Validity(Field {
                    raw_value:  profile::types::AttitudeValidity::decode::<T>(
                        buffer,
                    )?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            _ => {
                Ok(AviationAttitude::Unknown {
                    data: buffer.to_vec(),
                    field_def_num,
                })
            },
        }
    }
}
#[derive(Debug,Clone)]
pub enum Video {
    Url(Field<profile::base::Utf8String>),
    HostingProvider(Field<profile::base::Utf8String>),
    #[doc = "Playback time of video"]
    Duration(Field<profile::base::Uint32>),
    Unknown {
        data:          Vec<u8>,
        field_def_num: u8,
    },
}
impl Video {
    pub(crate) fn decode<T: ByteOrder>(
        buffer: &[u8],
        field_def_num: u8,
    ) -> error::Result<Self> {
        match field_def_num {
            0 => {
                Ok(Video::Url(Field {
                    raw_value:  profile::base::Utf8String::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            1 => {
                Ok(Video::HostingProvider(Field {
                    raw_value:  profile::base::Utf8String::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            2 => {
                Ok(Video::Duration(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("ms"),
                }))
            },
            _ => {
                Ok(Video::Unknown {
                    data: buffer.to_vec(),
                    field_def_num,
                })
            },
        }
    }
}
#[derive(Debug,Clone)]
pub enum VideoTitle {
    #[doc = "Long titles will be split into multiple parts"]
    MessageIndex(Field<profile::types::MessageIndex>),
    #[doc = "Total number of title parts"]
    MessageCount(Field<profile::base::Uint16>),
    Text(Field<profile::base::Utf8String>),
    Unknown {
        data:          Vec<u8>,
        field_def_num: u8,
    },
}
impl VideoTitle {
    pub(crate) fn decode<T: ByteOrder>(
        buffer: &[u8],
        field_def_num: u8,
    ) -> error::Result<Self> {
        match field_def_num {
            254 => {
                Ok(VideoTitle::MessageIndex(Field {
                    raw_value:  profile::types::MessageIndex::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            0 => {
                Ok(VideoTitle::MessageCount(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            1 => {
                Ok(VideoTitle::Text(Field {
                    raw_value:  profile::base::Utf8String::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            _ => {
                Ok(VideoTitle::Unknown {
                    data: buffer.to_vec(),
                    field_def_num,
                })
            },
        }
    }
}
#[derive(Debug,Clone)]
pub enum VideoDescription {
    #[doc = "Long descriptions will be split into multiple parts"]
    MessageIndex(Field<profile::types::MessageIndex>),
    #[doc = "Total number of description parts"]
    MessageCount(Field<profile::base::Uint16>),
    Text(Field<profile::base::Utf8String>),
    Unknown {
        data:          Vec<u8>,
        field_def_num: u8,
    },
}
impl VideoDescription {
    pub(crate) fn decode<T: ByteOrder>(
        buffer: &[u8],
        field_def_num: u8,
    ) -> error::Result<Self> {
        match field_def_num {
            254 => {
                Ok(VideoDescription::MessageIndex(Field {
                    raw_value:  profile::types::MessageIndex::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            0 => {
                Ok(VideoDescription::MessageCount(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            1 => {
                Ok(VideoDescription::Text(Field {
                    raw_value:  profile::base::Utf8String::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            _ => {
                Ok(VideoDescription::Unknown {
                    data: buffer.to_vec(),
                    field_def_num,
                })
            },
        }
    }
}
#[derive(Debug,Clone)]
pub enum VideoClip {
    ClipNumber(Field<profile::base::Uint16>),
    StartTimestamp(Field<profile::types::DateTime>),
    StartTimestampMs(Field<profile::base::Uint16>),
    EndTimestamp(Field<profile::types::DateTime>),
    EndTimestampMs(Field<profile::base::Uint16>),
    #[doc = "Start of clip in video time"]
    ClipStart(Field<profile::base::Uint32>),
    #[doc = "End of clip in video time"]
    ClipEnd(Field<profile::base::Uint32>),
    Unknown {
        data:          Vec<u8>,
        field_def_num: u8,
    },
}
impl VideoClip {
    pub(crate) fn decode<T: ByteOrder>(
        buffer: &[u8],
        field_def_num: u8,
    ) -> error::Result<Self> {
        match field_def_num {
            0 => {
                Ok(VideoClip::ClipNumber(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            1 => {
                Ok(VideoClip::StartTimestamp(Field {
                    raw_value:  profile::types::DateTime::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            2 => {
                Ok(VideoClip::StartTimestampMs(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            3 => {
                Ok(VideoClip::EndTimestamp(Field {
                    raw_value:  profile::types::DateTime::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            4 => {
                Ok(VideoClip::EndTimestampMs(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            6 => {
                Ok(VideoClip::ClipStart(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("ms"),
                }))
            },
            7 => {
                Ok(VideoClip::ClipEnd(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("ms"),
                }))
            },
            _ => {
                Ok(VideoClip::Unknown {
                    data: buffer.to_vec(),
                    field_def_num,
                })
            },
        }
    }
}
#[derive(Debug,Clone)]
pub enum Set {
    #[doc = "Timestamp of the set"]
    Timestamp(Field<profile::types::DateTime>),
    Duration(Field<profile::base::Uint32>),
    #[doc = "# of repitions of the movement"]
    Repetitions(Field<profile::base::Uint16>),
    #[doc = "Amount of weight applied for the set"]
    Weight(Field<profile::base::Uint16>),
    SetType(Field<profile::types::SetType>),
    #[doc = "Start time of the set"]
    StartTime(Field<profile::types::DateTime>),
    Category(Field<profile::types::ExerciseCategory>),
    #[doc = "Based on the associated category, see \
             \\[category\\]_exercise_names"]
    CategorySubtype(Field<profile::base::Uint16>),
    WeightDisplayUnit(Field<profile::types::FitBaseUnit>),
    MessageIndex(Field<profile::types::MessageIndex>),
    WktStepIndex(Field<profile::types::MessageIndex>),
    Unknown {
        data:          Vec<u8>,
        field_def_num: u8,
    },
}
impl Set {
    pub(crate) fn decode<T: ByteOrder>(
        buffer: &[u8],
        field_def_num: u8,
    ) -> error::Result<Self> {
        match field_def_num {
            254 => {
                Ok(Set::Timestamp(Field {
                    raw_value:  profile::types::DateTime::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            0 => {
                Ok(Set::Duration(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  Some(1000.0),
                    offset: None,
                    units:  Some("s"),
                }))
            },
            3 => {
                Ok(Set::Repetitions(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            4 => {
                Ok(Set::Weight(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  Some(16.0),
                    offset: None,
                    units:  Some("kg"),
                }))
            },
            5 => {
                Ok(Set::SetType(Field {
                    raw_value:  profile::types::SetType::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            6 => {
                Ok(Set::StartTime(Field {
                    raw_value:  profile::types::DateTime::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            7 => {
                Ok(Set::Category(Field {
                    raw_value:  profile::types::ExerciseCategory::decode::<T>(
                        buffer,
                    )?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            8 => {
                Ok(Set::CategorySubtype(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            9 => {
                Ok(Set::WeightDisplayUnit(Field {
                    raw_value:  profile::types::FitBaseUnit::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            10 => {
                Ok(Set::MessageIndex(Field {
                    raw_value:  profile::types::MessageIndex::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            11 => {
                Ok(Set::WktStepIndex(Field {
                    raw_value:  profile::types::MessageIndex::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            _ => {
                Ok(Set::Unknown {
                    data: buffer.to_vec(),
                    field_def_num,
                })
            },
        }
    }
}
#[derive(Debug,Clone)]
pub enum Course {
    Sport(Field<profile::types::Sport>),
    Name(Field<profile::base::Utf8String>),
    Capabilities(Field<profile::types::CourseCapabilities>),
    SubSport(Field<profile::types::SubSport>),
    Unknown { data:          Vec<u8>, field_def_num: u8 },
}
impl Course {
    pub(crate) fn decode<T: ByteOrder>(
        buffer: &[u8],
        field_def_num: u8,
    ) -> error::Result<Self> {
        match field_def_num {
            4 => {
                Ok(Course::Sport(Field {
                    raw_value:  profile::types::Sport::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            5 => {
                Ok(Course::Name(Field {
                    raw_value:  profile::base::Utf8String::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            6 => {
                Ok(Course::Capabilities(Field {
                    raw_value:  profile::types::CourseCapabilities::decode::<T>(
                        buffer,
                    )?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            7 => {
                Ok(Course::SubSport(Field {
                    raw_value:  profile::types::SubSport::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            _ => {
                Ok(Course::Unknown {
                    data: buffer.to_vec(),
                    field_def_num,
                })
            },
        }
    }
}
#[derive(Debug,Clone)]
pub enum CoursePoint {
    MessageIndex(Field<profile::types::MessageIndex>),
    Timestamp(Field<profile::types::DateTime>),
    PositionLat(Field<profile::base::Sint32>),
    PositionLong(Field<profile::base::Sint32>),
    Distance(Field<profile::base::Uint32>),
    Type(Field<profile::types::CoursePoint>),
    Name(Field<profile::base::Utf8String>),
    Favorite(Field<profile::base::Bool>),
    Unknown { data:          Vec<u8>, field_def_num: u8 },
}
impl CoursePoint {
    pub(crate) fn decode<T: ByteOrder>(
        buffer: &[u8],
        field_def_num: u8,
    ) -> error::Result<Self> {
        match field_def_num {
            254 => {
                Ok(CoursePoint::MessageIndex(Field {
                    raw_value:  profile::types::MessageIndex::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            1 => {
                Ok(CoursePoint::Timestamp(Field {
                    raw_value:  profile::types::DateTime::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            2 => {
                Ok(CoursePoint::PositionLat(Field {
                    raw_value:  profile::base::Sint32::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("semicircles"),
                }))
            },
            3 => {
                Ok(CoursePoint::PositionLong(Field {
                    raw_value:  profile::base::Sint32::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("semicircles"),
                }))
            },
            4 => {
                Ok(CoursePoint::Distance(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  Some(100.0),
                    offset: None,
                    units:  Some("m"),
                }))
            },
            5 => {
                Ok(CoursePoint::Type(Field {
                    raw_value:  profile::types::CoursePoint::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            6 => {
                Ok(CoursePoint::Name(Field {
                    raw_value:  profile::base::Utf8String::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            8 => {
                Ok(CoursePoint::Favorite(Field {
                    raw_value:  profile::base::Bool::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            _ => {
                Ok(CoursePoint::Unknown {
                    data: buffer.to_vec(),
                    field_def_num,
                })
            },
        }
    }
}
#[doc = "Unique Identification data for a segment file"]
#[derive(Debug,Clone)]
pub enum SegmentId {
    #[doc = "Friendly name assigned to segment"]
    Name(Field<profile::base::Utf8String>),
    #[doc = "UUID of the segment"]
    Uuid(Field<profile::base::Utf8String>),
    #[doc = "Sport associated with the segment"]
    Sport(Field<profile::types::Sport>),
    #[doc = "Segment enabled for evaluation"]
    Enabled(Field<profile::base::Bool>),
    #[doc = "Primary key of the user that created the segment"]
    UserProfilePrimaryKey(Field<profile::base::Uint32>),
    #[doc = "ID of the device that created the segment"]
    DeviceId(Field<profile::base::Uint32>),
    #[doc = "Index for the Leader Board entry selected as the default race \
             participant"]
    DefaultRaceLeader(Field<profile::base::Uint8>),
    #[doc = "Indicates if any segments should be deleted"]
    DeleteStatus(Field<profile::types::SegmentDeleteStatus>),
    #[doc = "Indicates how the segment was selected to be sent to the device"]
    SelectionType(Field<profile::types::SegmentSelectionType>),
    Unknown {
        data:          Vec<u8>,
        field_def_num: u8,
    },
}
impl SegmentId {
    pub(crate) fn decode<T: ByteOrder>(
        buffer: &[u8],
        field_def_num: u8,
    ) -> error::Result<Self> {
        match field_def_num {
            0 => {
                Ok(SegmentId::Name(Field {
                    raw_value:  profile::base::Utf8String::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            1 => {
                Ok(SegmentId::Uuid(Field {
                    raw_value:  profile::base::Utf8String::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            2 => {
                Ok(SegmentId::Sport(Field {
                    raw_value:  profile::types::Sport::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            3 => {
                Ok(SegmentId::Enabled(Field {
                    raw_value:  profile::base::Bool::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            4 => {
                Ok(SegmentId::UserProfilePrimaryKey(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            5 => {
                Ok(SegmentId::DeviceId(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            6 => {
                Ok(SegmentId::DefaultRaceLeader(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            7 => {
                Ok(SegmentId::DeleteStatus(Field {
                    raw_value:  profile::types::SegmentDeleteStatus::decode::<T>(
                        buffer,
                    )?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            8 => {
                Ok(SegmentId::SelectionType(Field {
                    raw_value:  profile::types::SegmentSelectionType::decode::<T>(
                        buffer,
                    )?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            _ => {
                Ok(SegmentId::Unknown {
                    data: buffer.to_vec(),
                    field_def_num,
                })
            },
        }
    }
}
#[doc = "Unique Identification data for an individual segment leader within a \
         segment file"]
#[derive(Debug,Clone)]
pub enum SegmentLeaderboardEntry {
    MessageIndex(Field<profile::types::MessageIndex>),
    #[doc = "Friendly name assigned to leader"]
    Name(Field<profile::base::Utf8String>),
    #[doc = "Leader classification"]
    Type(Field<profile::types::SegmentLeaderboardType>),
    #[doc = "Primary user ID of this leader"]
    GroupPrimaryKey(Field<profile::base::Uint32>),
    #[doc = "ID of the activity associated with this leader time"]
    ActivityId(Field<profile::base::Uint32>),
    #[doc = "Segment Time (includes pauses)"]
    SegmentTime(Field<profile::base::Uint32>),
    #[doc = "String version of the activity_id. 21 characters long, express \
             in decimal"]
    ActivityIdString(Field<profile::base::Utf8String>),
    Unknown {
        data:          Vec<u8>,
        field_def_num: u8,
    },
}
impl SegmentLeaderboardEntry {
    pub(crate) fn decode<T: ByteOrder>(
        buffer: &[u8],
        field_def_num: u8,
    ) -> error::Result<Self> {
        match field_def_num {
            254 => {
                Ok(SegmentLeaderboardEntry::MessageIndex(Field {
                    raw_value:  profile::types::MessageIndex::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            0 => {
                Ok(SegmentLeaderboardEntry::Name(Field {
                    raw_value:  profile::base::Utf8String::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            1 => {
                Ok(SegmentLeaderboardEntry::Type(Field {
                    raw_value:  profile::types::SegmentLeaderboardType::decode::<T>(
                        buffer,
                    )?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            2 => {
                Ok(SegmentLeaderboardEntry::GroupPrimaryKey(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            3 => {
                Ok(SegmentLeaderboardEntry::ActivityId(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            4 => {
                Ok(SegmentLeaderboardEntry::SegmentTime(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  Some(1000.0),
                    offset: None,
                    units:  Some("s"),
                }))
            },
            5 => {
                Ok(SegmentLeaderboardEntry::ActivityIdString(Field {
                    raw_value:  profile::base::Utf8String::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            _ => {
                Ok(SegmentLeaderboardEntry::Unknown {
                    data: buffer.to_vec(),
                    field_def_num,
                })
            },
        }
    }
}
#[doc = "Navigation and race evaluation point for a segment decribing a point \
         along the segment path and time it took each segment leader to reach \
         that point"]
#[derive(Debug,Clone)]
pub enum SegmentPoint {
    MessageIndex(Field<profile::types::MessageIndex>),
    PositionLat(Field<profile::base::Sint32>),
    PositionLong(Field<profile::base::Sint32>),
    #[doc = "Accumulated distance along the segment at the described point"]
    Distance(Field<profile::base::Uint32>),
    #[doc = "Accumulated altitude along the segment at the described point"]
    Altitude(Field<profile::base::Uint16>),
    #[doc = "Accumualted time each leader board member required to reach the \
             described point. This raw_value is zero for all leader board members \
             at the starting point of the segment."]
    LeaderTime(Field<profile::base::Uint32>),
    Unknown {
        data:          Vec<u8>,
        field_def_num: u8,
    },
}
impl SegmentPoint {
    pub(crate) fn decode<T: ByteOrder>(
        buffer: &[u8],
        field_def_num: u8,
    ) -> error::Result<Self> {
        match field_def_num {
            254 => {
                Ok(SegmentPoint::MessageIndex(Field {
                    raw_value:  profile::types::MessageIndex::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            1 => {
                Ok(SegmentPoint::PositionLat(Field {
                    raw_value:  profile::base::Sint32::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("semicircles"),
                }))
            },
            2 => {
                Ok(SegmentPoint::PositionLong(Field {
                    raw_value:  profile::base::Sint32::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("semicircles"),
                }))
            },
            3 => {
                Ok(SegmentPoint::Distance(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  Some(100.0),
                    offset: None,
                    units:  Some("m"),
                }))
            },
            4 => {
                Ok(SegmentPoint::Altitude(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  Some(5.0),
                    offset: Some(500.0),
                    units:  Some("m"),
                }))
            },
            5 => {
                Ok(SegmentPoint::LeaderTime(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  Some(1000.0),
                    offset: None,
                    units:  Some("s"),
                }))
            },
            _ => {
                Ok(SegmentPoint::Unknown {
                    data: buffer.to_vec(),
                    field_def_num,
                })
            },
        }
    }
}
#[derive(Debug,Clone)]
pub enum SegmentLap {
    MessageIndex(Field<profile::types::MessageIndex>),
    #[doc = "Lap end time."]
    Timestamp(Field<profile::types::DateTime>),
    Event(Field<profile::types::Event>),
    EventType(Field<profile::types::EventType>),
    StartTime(Field<profile::types::DateTime>),
    StartPositionLat(Field<profile::base::Sint32>),
    StartPositionLong(Field<profile::base::Sint32>),
    EndPositionLat(Field<profile::base::Sint32>),
    EndPositionLong(Field<profile::base::Sint32>),
    #[doc = "Time (includes pauses)"]
    TotalElapsedTime(Field<profile::base::Uint32>),
    #[doc = "Timer Time (excludes pauses)"]
    TotalTimerTime(Field<profile::base::Uint32>),
    TotalDistance(Field<profile::base::Uint32>),
    TotalCycles(Field<profile::base::Uint32>),
    TotalCalories(Field<profile::base::Uint16>),
    #[doc = "If New Leaf"]
    TotalFatCalories(Field<profile::base::Uint16>),
    AvgSpeed(Field<profile::base::Uint16>),
    MaxSpeed(Field<profile::base::Uint16>),
    AvgHeartRate(Field<profile::base::Uint8>),
    MaxHeartRate(Field<profile::base::Uint8>),
    #[doc = "total_cycles / total_timer_time if non_zero_avg_cadence \
             otherwise total_cycles / total_elapsed_time"]
    AvgCadence(Field<profile::base::Uint8>),
    MaxCadence(Field<profile::base::Uint8>),
    #[doc = "total_power / total_timer_time if non_zero_avg_power otherwise \
             total_power / total_elapsed_time"]
    AvgPower(Field<profile::base::Uint16>),
    MaxPower(Field<profile::base::Uint16>),
    TotalAscent(Field<profile::base::Uint16>),
    TotalDescent(Field<profile::base::Uint16>),
    Sport(Field<profile::types::Sport>),
    EventGroup(Field<profile::base::Uint8>),
    #[doc = "North east corner latitude."]
    NecLat(Field<profile::base::Sint32>),
    #[doc = "North east corner longitude."]
    NecLong(Field<profile::base::Sint32>),
    #[doc = "South west corner latitude."]
    SwcLat(Field<profile::base::Sint32>),
    #[doc = "South west corner latitude."]
    SwcLong(Field<profile::base::Sint32>),
    Name(Field<profile::base::Utf8String>),
    NormalizedPower(Field<profile::base::Uint16>),
    LeftRightBalance(Field<profile::types::LeftRightBalance100>),
    SubSport(Field<profile::types::SubSport>),
    TotalWork(Field<profile::base::Uint32>),
    AvgAltitude(Field<profile::base::Uint16>),
    MaxAltitude(Field<profile::base::Uint16>),
    GpsAccuracy(Field<profile::base::Uint8>),
    AvgGrade(Field<profile::base::Sint16>),
    AvgPosGrade(Field<profile::base::Sint16>),
    AvgNegGrade(Field<profile::base::Sint16>),
    MaxPosGrade(Field<profile::base::Sint16>),
    MaxNegGrade(Field<profile::base::Sint16>),
    AvgTemperature(Field<profile::base::Sint8>),
    MaxTemperature(Field<profile::base::Sint8>),
    TotalMovingTime(Field<profile::base::Uint32>),
    AvgPosVerticalSpeed(Field<profile::base::Sint16>),
    AvgNegVerticalSpeed(Field<profile::base::Sint16>),
    MaxPosVerticalSpeed(Field<profile::base::Sint16>),
    MaxNegVerticalSpeed(Field<profile::base::Sint16>),
    TimeInHrZone(Field<profile::base::Uint32>),
    TimeInSpeedZone(Field<profile::base::Uint32>),
    TimeInCadenceZone(Field<profile::base::Uint32>),
    TimeInPowerZone(Field<profile::base::Uint32>),
    RepetitionNum(Field<profile::base::Uint16>),
    MinAltitude(Field<profile::base::Uint16>),
    MinHeartRate(Field<profile::base::Uint8>),
    ActiveTime(Field<profile::base::Uint32>),
    WktStepIndex(Field<profile::types::MessageIndex>),
    SportEvent(Field<profile::types::SportEvent>),
    AvgLeftTorqueEffectiveness(Field<profile::base::Uint8>),
    AvgRightTorqueEffectiveness(Field<profile::base::Uint8>),
    AvgLeftPedalSmoothness(Field<profile::base::Uint8>),
    AvgRightPedalSmoothness(Field<profile::base::Uint8>),
    AvgCombinedPedalSmoothness(Field<profile::base::Uint8>),
    Status(Field<profile::types::SegmentLapStatus>),
    Uuid(Field<profile::base::Utf8String>),
    #[doc = "fractional part of the avg_cadence"]
    AvgFractionalCadence(Field<profile::base::Uint8>),
    #[doc = "fractional part of the max_cadence"]
    MaxFractionalCadence(Field<profile::base::Uint8>),
    #[doc = "fractional part of the total_cycles"]
    TotalFractionalCycles(Field<profile::base::Uint8>),
    FrontGearShiftCount(Field<profile::base::Uint16>),
    RearGearShiftCount(Field<profile::base::Uint16>),
    #[doc = "Total time spent in the standing position"]
    TimeStanding(Field<profile::base::Uint32>),
    #[doc = "Number of transitions to the standing state"]
    StandCount(Field<profile::base::Uint16>),
    #[doc = "Average left platform center offset"]
    AvgLeftPco(Field<profile::base::Sint8>),
    #[doc = "Average right platform center offset"]
    AvgRightPco(Field<profile::base::Sint8>),
    #[doc = "Average left power phase angles. Data raw_value indexes defined by \
             power_phase_type."]
    AvgLeftPowerPhase(Field<profile::base::Uint8>),
    #[doc = "Average left power phase peak angles. Data raw_value indexes defined \
             by power_phase_type."]
    AvgLeftPowerPhasePeak(Field<profile::base::Uint8>),
    #[doc = "Average right power phase angles. Data raw_value indexes defined by \
             power_phase_type."]
    AvgRightPowerPhase(Field<profile::base::Uint8>),
    #[doc = "Average right power phase peak angles. Data raw_value indexes \
             defined by power_phase_type."]
    AvgRightPowerPhasePeak(Field<profile::base::Uint8>),
    #[doc = "Average power by position. Data raw_value indexes defined by \
             rider_position_type."]
    AvgPowerPosition(Field<profile::base::Uint16>),
    #[doc = "Maximum power by position. Data raw_value indexes defined by \
             rider_position_type."]
    MaxPowerPosition(Field<profile::base::Uint16>),
    #[doc = "Average cadence by position. Data raw_value indexes defined by \
             rider_position_type."]
    AvgCadencePosition(Field<profile::base::Uint8>),
    #[doc = "Maximum cadence by position. Data raw_value indexes defined by \
             rider_position_type."]
    MaxCadencePosition(Field<profile::base::Uint8>),
    #[doc = "Manufacturer that produced the segment"]
    Manufacturer(Field<profile::types::Manufacturer>),
    Unknown {
        data:          Vec<u8>,
        field_def_num: u8,
    },
}
impl SegmentLap {
    pub(crate) fn decode<T: ByteOrder>(
        buffer: &[u8],
        field_def_num: u8,
    ) -> error::Result<Self> {
        match field_def_num {
            254 => {
                Ok(SegmentLap::MessageIndex(Field {
                    raw_value:  profile::types::MessageIndex::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            253 => {
                Ok(SegmentLap::Timestamp(Field {
                    raw_value:  profile::types::DateTime::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("s"),
                }))
            },
            0 => {
                Ok(SegmentLap::Event(Field {
                    raw_value:  profile::types::Event::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            1 => {
                Ok(SegmentLap::EventType(Field {
                    raw_value:  profile::types::EventType::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            2 => {
                Ok(SegmentLap::StartTime(Field {
                    raw_value:  profile::types::DateTime::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            3 => {
                Ok(SegmentLap::StartPositionLat(Field {
                    raw_value:  profile::base::Sint32::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("semicircles"),
                }))
            },
            4 => {
                Ok(SegmentLap::StartPositionLong(Field {
                    raw_value:  profile::base::Sint32::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("semicircles"),
                }))
            },
            5 => {
                Ok(SegmentLap::EndPositionLat(Field {
                    raw_value:  profile::base::Sint32::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("semicircles"),
                }))
            },
            6 => {
                Ok(SegmentLap::EndPositionLong(Field {
                    raw_value:  profile::base::Sint32::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("semicircles"),
                }))
            },
            7 => {
                Ok(SegmentLap::TotalElapsedTime(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  Some(1000.0),
                    offset: None,
                    units:  Some("s"),
                }))
            },
            8 => {
                Ok(SegmentLap::TotalTimerTime(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  Some(1000.0),
                    offset: None,
                    units:  Some("s"),
                }))
            },
            9 => {
                Ok(SegmentLap::TotalDistance(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  Some(100.0),
                    offset: None,
                    units:  Some("m"),
                }))
            },
            10 => {
                Ok(SegmentLap::TotalCycles(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("cycles"),
                }))
            },
            11 => {
                Ok(SegmentLap::TotalCalories(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("kcal"),
                }))
            },
            12 => {
                Ok(SegmentLap::TotalFatCalories(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("kcal"),
                }))
            },
            13 => {
                Ok(SegmentLap::AvgSpeed(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  Some(1000.0),
                    offset: None,
                    units:  Some("m/s"),
                }))
            },
            14 => {
                Ok(SegmentLap::MaxSpeed(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  Some(1000.0),
                    offset: None,
                    units:  Some("m/s"),
                }))
            },
            15 => {
                Ok(SegmentLap::AvgHeartRate(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("bpm"),
                }))
            },
            16 => {
                Ok(SegmentLap::MaxHeartRate(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("bpm"),
                }))
            },
            17 => {
                Ok(SegmentLap::AvgCadence(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("rpm"),
                }))
            },
            18 => {
                Ok(SegmentLap::MaxCadence(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("rpm"),
                }))
            },
            19 => {
                Ok(SegmentLap::AvgPower(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("watts"),
                }))
            },
            20 => {
                Ok(SegmentLap::MaxPower(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("watts"),
                }))
            },
            21 => {
                Ok(SegmentLap::TotalAscent(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("m"),
                }))
            },
            22 => {
                Ok(SegmentLap::TotalDescent(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("m"),
                }))
            },
            23 => {
                Ok(SegmentLap::Sport(Field {
                    raw_value:  profile::types::Sport::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            24 => {
                Ok(SegmentLap::EventGroup(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            25 => {
                Ok(SegmentLap::NecLat(Field {
                    raw_value:  profile::base::Sint32::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("semicircles"),
                }))
            },
            26 => {
                Ok(SegmentLap::NecLong(Field {
                    raw_value:  profile::base::Sint32::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("semicircles"),
                }))
            },
            27 => {
                Ok(SegmentLap::SwcLat(Field {
                    raw_value:  profile::base::Sint32::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("semicircles"),
                }))
            },
            28 => {
                Ok(SegmentLap::SwcLong(Field {
                    raw_value:  profile::base::Sint32::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("semicircles"),
                }))
            },
            29 => {
                Ok(SegmentLap::Name(Field {
                    raw_value:  profile::base::Utf8String::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            30 => {
                Ok(SegmentLap::NormalizedPower(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("watts"),
                }))
            },
            31 => {
                Ok(SegmentLap::LeftRightBalance(Field {
                    raw_value:  profile::types::LeftRightBalance100::decode::<T>(
                        buffer,
                    )?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            32 => {
                Ok(SegmentLap::SubSport(Field {
                    raw_value:  profile::types::SubSport::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            33 => {
                Ok(SegmentLap::TotalWork(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("J"),
                }))
            },
            34 => {
                Ok(SegmentLap::AvgAltitude(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  Some(5.0),
                    offset: Some(500.0),
                    units:  Some("m"),
                }))
            },
            35 => {
                Ok(SegmentLap::MaxAltitude(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  Some(5.0),
                    offset: Some(500.0),
                    units:  Some("m"),
                }))
            },
            36 => {
                Ok(SegmentLap::GpsAccuracy(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("m"),
                }))
            },
            37 => {
                Ok(SegmentLap::AvgGrade(Field {
                    raw_value:  profile::base::Sint16::decode::<T>(buffer)?,
                    scale:  Some(100.0),
                    offset: None,
                    units:  Some("%"),
                }))
            },
            38 => {
                Ok(SegmentLap::AvgPosGrade(Field {
                    raw_value:  profile::base::Sint16::decode::<T>(buffer)?,
                    scale:  Some(100.0),
                    offset: None,
                    units:  Some("%"),
                }))
            },
            39 => {
                Ok(SegmentLap::AvgNegGrade(Field {
                    raw_value:  profile::base::Sint16::decode::<T>(buffer)?,
                    scale:  Some(100.0),
                    offset: None,
                    units:  Some("%"),
                }))
            },
            40 => {
                Ok(SegmentLap::MaxPosGrade(Field {
                    raw_value:  profile::base::Sint16::decode::<T>(buffer)?,
                    scale:  Some(100.0),
                    offset: None,
                    units:  Some("%"),
                }))
            },
            41 => {
                Ok(SegmentLap::MaxNegGrade(Field {
                    raw_value:  profile::base::Sint16::decode::<T>(buffer)?,
                    scale:  Some(100.0),
                    offset: None,
                    units:  Some("%"),
                }))
            },
            42 => {
                Ok(SegmentLap::AvgTemperature(Field {
                    raw_value:  profile::base::Sint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("C"),
                }))
            },
            43 => {
                Ok(SegmentLap::MaxTemperature(Field {
                    raw_value:  profile::base::Sint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("C"),
                }))
            },
            44 => {
                Ok(SegmentLap::TotalMovingTime(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  Some(1000.0),
                    offset: None,
                    units:  Some("s"),
                }))
            },
            45 => {
                Ok(SegmentLap::AvgPosVerticalSpeed(Field {
                    raw_value:  profile::base::Sint16::decode::<T>(buffer)?,
                    scale:  Some(1000.0),
                    offset: None,
                    units:  Some("m/s"),
                }))
            },
            46 => {
                Ok(SegmentLap::AvgNegVerticalSpeed(Field {
                    raw_value:  profile::base::Sint16::decode::<T>(buffer)?,
                    scale:  Some(1000.0),
                    offset: None,
                    units:  Some("m/s"),
                }))
            },
            47 => {
                Ok(SegmentLap::MaxPosVerticalSpeed(Field {
                    raw_value:  profile::base::Sint16::decode::<T>(buffer)?,
                    scale:  Some(1000.0),
                    offset: None,
                    units:  Some("m/s"),
                }))
            },
            48 => {
                Ok(SegmentLap::MaxNegVerticalSpeed(Field {
                    raw_value:  profile::base::Sint16::decode::<T>(buffer)?,
                    scale:  Some(1000.0),
                    offset: None,
                    units:  Some("m/s"),
                }))
            },
            49 => {
                Ok(SegmentLap::TimeInHrZone(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  Some(1000.0),
                    offset: None,
                    units:  Some("s"),
                }))
            },
            50 => {
                Ok(SegmentLap::TimeInSpeedZone(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  Some(1000.0),
                    offset: None,
                    units:  Some("s"),
                }))
            },
            51 => {
                Ok(SegmentLap::TimeInCadenceZone(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  Some(1000.0),
                    offset: None,
                    units:  Some("s"),
                }))
            },
            52 => {
                Ok(SegmentLap::TimeInPowerZone(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  Some(1000.0),
                    offset: None,
                    units:  Some("s"),
                }))
            },
            53 => {
                Ok(SegmentLap::RepetitionNum(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            54 => {
                Ok(SegmentLap::MinAltitude(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  Some(5.0),
                    offset: Some(500.0),
                    units:  Some("m"),
                }))
            },
            55 => {
                Ok(SegmentLap::MinHeartRate(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("bpm"),
                }))
            },
            56 => {
                Ok(SegmentLap::ActiveTime(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  Some(1000.0),
                    offset: None,
                    units:  Some("s"),
                }))
            },
            57 => {
                Ok(SegmentLap::WktStepIndex(Field {
                    raw_value:  profile::types::MessageIndex::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            58 => {
                Ok(SegmentLap::SportEvent(Field {
                    raw_value:  profile::types::SportEvent::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            59 => {
                Ok(SegmentLap::AvgLeftTorqueEffectiveness(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  Some(2.0),
                    offset: None,
                    units:  Some("percent"),
                }))
            },
            60 => {
                Ok(SegmentLap::AvgRightTorqueEffectiveness(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  Some(2.0),
                    offset: None,
                    units:  Some("percent"),
                }))
            },
            61 => {
                Ok(SegmentLap::AvgLeftPedalSmoothness(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  Some(2.0),
                    offset: None,
                    units:  Some("percent"),
                }))
            },
            62 => {
                Ok(SegmentLap::AvgRightPedalSmoothness(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  Some(2.0),
                    offset: None,
                    units:  Some("percent"),
                }))
            },
            63 => {
                Ok(SegmentLap::AvgCombinedPedalSmoothness(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  Some(2.0),
                    offset: None,
                    units:  Some("percent"),
                }))
            },
            64 => {
                Ok(SegmentLap::Status(Field {
                    raw_value:  profile::types::SegmentLapStatus::decode::<T>(
                        buffer,
                    )?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            65 => {
                Ok(SegmentLap::Uuid(Field {
                    raw_value:  profile::base::Utf8String::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            66 => {
                Ok(SegmentLap::AvgFractionalCadence(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  Some(128.0),
                    offset: None,
                    units:  Some("rpm"),
                }))
            },
            67 => {
                Ok(SegmentLap::MaxFractionalCadence(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  Some(128.0),
                    offset: None,
                    units:  Some("rpm"),
                }))
            },
            68 => {
                Ok(SegmentLap::TotalFractionalCycles(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  Some(128.0),
                    offset: None,
                    units:  Some("cycles"),
                }))
            },
            69 => {
                Ok(SegmentLap::FrontGearShiftCount(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            70 => {
                Ok(SegmentLap::RearGearShiftCount(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            71 => {
                Ok(SegmentLap::TimeStanding(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  Some(1000.0),
                    offset: None,
                    units:  Some("s"),
                }))
            },
            72 => {
                Ok(SegmentLap::StandCount(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            73 => {
                Ok(SegmentLap::AvgLeftPco(Field {
                    raw_value:  profile::base::Sint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("mm"),
                }))
            },
            74 => {
                Ok(SegmentLap::AvgRightPco(Field {
                    raw_value:  profile::base::Sint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("mm"),
                }))
            },
            75 => {
                Ok(SegmentLap::AvgLeftPowerPhase(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  Some(0.7111111),
                    offset: None,
                    units:  Some("degrees"),
                }))
            },
            76 => {
                Ok(SegmentLap::AvgLeftPowerPhasePeak(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  Some(0.7111111),
                    offset: None,
                    units:  Some("degrees"),
                }))
            },
            77 => {
                Ok(SegmentLap::AvgRightPowerPhase(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  Some(0.7111111),
                    offset: None,
                    units:  Some("degrees"),
                }))
            },
            78 => {
                Ok(SegmentLap::AvgRightPowerPhasePeak(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  Some(0.7111111),
                    offset: None,
                    units:  Some("degrees"),
                }))
            },
            79 => {
                Ok(SegmentLap::AvgPowerPosition(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("watts"),
                }))
            },
            80 => {
                Ok(SegmentLap::MaxPowerPosition(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("watts"),
                }))
            },
            81 => {
                Ok(SegmentLap::AvgCadencePosition(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("rpm"),
                }))
            },
            82 => {
                Ok(SegmentLap::MaxCadencePosition(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("rpm"),
                }))
            },
            83 => {
                Ok(SegmentLap::Manufacturer(Field {
                    raw_value:  profile::types::Manufacturer::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            _ => {
                Ok(SegmentLap::Unknown {
                    data: buffer.to_vec(),
                    field_def_num,
                })
            },
        }
    }
}
#[doc = "Summary of the unique segment and leaderboard information associated \
         with a segment file. This message is used to compile a segment list \
         file describing all segment files on a device. The segment list file \
         is used when refreshing the contents of a segment file with the \
         latest available leaderboard information."]
#[derive(Debug,Clone)]
pub enum SegmentFile {
    MessageIndex(Field<profile::types::MessageIndex>),
    #[doc = "UUID of the segment file"]
    FileUuid(Field<profile::base::Utf8String>),
    #[doc = "Enabled state of the segment file"]
    Enabled(Field<profile::base::Bool>),
    #[doc = "Primary key of the user that created the segment file"]
    UserProfilePrimaryKey(Field<profile::base::Uint32>),
    #[doc = "Leader type of each leader in the segment file"]
    LeaderType(Field<profile::types::SegmentLeaderboardType>),
    #[doc = "Group primary key of each leader in the segment file"]
    LeaderGroupPrimaryKey(Field<profile::base::Uint32>),
    #[doc = "Activity ID of each leader in the segment file"]
    LeaderActivityId(Field<profile::base::Uint32>),
    #[doc = "String version of the activity ID of each leader in the segment \
             file. 21 characters long for each ID, express in decimal"]
    LeaderActivityIdString(Field<profile::base::Utf8String>),
    #[doc = "Index for the Leader Board entry selected as the default race \
             participant"]
    DefaultRaceLeader(Field<profile::base::Uint8>),
    Unknown {
        data:          Vec<u8>,
        field_def_num: u8,
    },
}
impl SegmentFile {
    pub(crate) fn decode<T: ByteOrder>(
        buffer: &[u8],
        field_def_num: u8,
    ) -> error::Result<Self> {
        match field_def_num {
            254 => {
                Ok(SegmentFile::MessageIndex(Field {
                    raw_value:  profile::types::MessageIndex::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            1 => {
                Ok(SegmentFile::FileUuid(Field {
                    raw_value:  profile::base::Utf8String::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            3 => {
                Ok(SegmentFile::Enabled(Field {
                    raw_value:  profile::base::Bool::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            4 => {
                Ok(SegmentFile::UserProfilePrimaryKey(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            7 => {
                Ok(SegmentFile::LeaderType(Field {
                    raw_value:  profile::types::SegmentLeaderboardType::decode::<T>(
                        buffer,
                    )?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            8 => {
                Ok(SegmentFile::LeaderGroupPrimaryKey(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            9 => {
                Ok(SegmentFile::LeaderActivityId(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            10 => {
                Ok(SegmentFile::LeaderActivityIdString(Field {
                    raw_value:  profile::base::Utf8String::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            11 => {
                Ok(SegmentFile::DefaultRaceLeader(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            _ => {
                Ok(SegmentFile::Unknown {
                    data: buffer.to_vec(),
                    field_def_num,
                })
            },
        }
    }
}
#[derive(Debug,Clone)]
pub enum Workout {
    Sport(Field<profile::types::Sport>),
    Capabilities(Field<profile::types::WorkoutCapabilities>),
    #[doc = "number of valid steps"]
    NumValidSteps(Field<profile::base::Uint16>),
    WktName(Field<profile::base::Utf8String>),
    SubSport(Field<profile::types::SubSport>),
    PoolLength(Field<profile::base::Uint16>),
    PoolLengthUnit(Field<profile::types::DisplayMeasure>),
    Unknown {
        data:          Vec<u8>,
        field_def_num: u8,
    },
}
impl Workout {
    pub(crate) fn decode<T: ByteOrder>(
        buffer: &[u8],
        field_def_num: u8,
    ) -> error::Result<Self> {
        match field_def_num {
            4 => {
                Ok(Workout::Sport(Field {
                    raw_value:  profile::types::Sport::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            5 => {
                Ok(Workout::Capabilities(Field {
                    raw_value:  profile::types::WorkoutCapabilities::decode::<T>(
                        buffer,
                    )?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            6 => {
                Ok(Workout::NumValidSteps(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            8 => {
                Ok(Workout::WktName(Field {
                    raw_value:  profile::base::Utf8String::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            11 => {
                Ok(Workout::SubSport(Field {
                    raw_value:  profile::types::SubSport::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            14 => {
                Ok(Workout::PoolLength(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  Some(100.0),
                    offset: None,
                    units:  Some("m"),
                }))
            },
            15 => {
                Ok(Workout::PoolLengthUnit(Field {
                    raw_value:  profile::types::DisplayMeasure::decode::<T>(
                        buffer,
                    )?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            _ => {
                Ok(Workout::Unknown {
                    data: buffer.to_vec(),
                    field_def_num,
                })
            },
        }
    }
}
#[derive(Debug,Clone)]
pub enum WorkoutSession {
    MessageIndex(Field<profile::types::MessageIndex>),
    Sport(Field<profile::types::Sport>),
    SubSport(Field<profile::types::SubSport>),
    NumValidSteps(Field<profile::base::Uint16>),
    FirstStepIndex(Field<profile::base::Uint16>),
    PoolLength(Field<profile::base::Uint16>),
    PoolLengthUnit(Field<profile::types::DisplayMeasure>),
    Unknown { data:          Vec<u8>, field_def_num: u8 },
}
impl WorkoutSession {
    pub(crate) fn decode<T: ByteOrder>(
        buffer: &[u8],
        field_def_num: u8,
    ) -> error::Result<Self> {
        match field_def_num {
            254 => {
                Ok(WorkoutSession::MessageIndex(Field {
                    raw_value:  profile::types::MessageIndex::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            0 => {
                Ok(WorkoutSession::Sport(Field {
                    raw_value:  profile::types::Sport::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            1 => {
                Ok(WorkoutSession::SubSport(Field {
                    raw_value:  profile::types::SubSport::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            2 => {
                Ok(WorkoutSession::NumValidSteps(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            3 => {
                Ok(WorkoutSession::FirstStepIndex(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            4 => {
                Ok(WorkoutSession::PoolLength(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  Some(100.0),
                    offset: None,
                    units:  Some("m"),
                }))
            },
            5 => {
                Ok(WorkoutSession::PoolLengthUnit(Field {
                    raw_value:  profile::types::DisplayMeasure::decode::<T>(
                        buffer,
                    )?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            _ => {
                Ok(WorkoutSession::Unknown {
                    data: buffer.to_vec(),
                    field_def_num,
                })
            },
        }
    }
}
#[derive(Debug,Clone)]
pub enum WorkoutStep {
    MessageIndex(Field<profile::types::MessageIndex>),
    WktStepName(Field<profile::base::Utf8String>),
    DurationType(Field<profile::types::WktStepDuration>),
    DurationValue(Field<profile::base::Uint32>),
    TargetType(Field<profile::types::WktStepTarget>),
    TargetValue(Field<profile::base::Uint32>),
    CustomTargetValueLow(Field<profile::base::Uint32>),
    CustomTargetValueHigh(Field<profile::base::Uint32>),
    Intensity(Field<profile::types::Intensity>),
    Notes(Field<profile::base::Utf8String>),
    Equipment(Field<profile::types::WorkoutEquipment>),
    ExerciseCategory(Field<profile::types::ExerciseCategory>),
    ExerciseName(Field<profile::base::Uint16>),
    ExerciseWeight(Field<profile::base::Uint16>),
    WeightDisplayUnit(Field<profile::types::FitBaseUnit>),
    Unknown { data:          Vec<u8>, field_def_num: u8 },
}
impl WorkoutStep {
    pub(crate) fn decode<T: ByteOrder>(
        buffer: &[u8],
        field_def_num: u8,
    ) -> error::Result<Self> {
        match field_def_num {
            254 => {
                Ok(WorkoutStep::MessageIndex(Field {
                    raw_value:  profile::types::MessageIndex::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            0 => {
                Ok(WorkoutStep::WktStepName(Field {
                    raw_value:  profile::base::Utf8String::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            1 => {
                Ok(WorkoutStep::DurationType(Field {
                    raw_value:  profile::types::WktStepDuration::decode::<T>(
                        buffer,
                    )?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            2 => {
                Ok(WorkoutStep::DurationValue(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            3 => {
                Ok(WorkoutStep::TargetType(Field {
                    raw_value:  profile::types::WktStepTarget::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            4 => {
                Ok(WorkoutStep::TargetValue(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            5 => {
                Ok(WorkoutStep::CustomTargetValueLow(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            6 => {
                Ok(WorkoutStep::CustomTargetValueHigh(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            7 => {
                Ok(WorkoutStep::Intensity(Field {
                    raw_value:  profile::types::Intensity::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            8 => {
                Ok(WorkoutStep::Notes(Field {
                    raw_value:  profile::base::Utf8String::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            9 => {
                Ok(WorkoutStep::Equipment(Field {
                    raw_value:  profile::types::WorkoutEquipment::decode::<T>(
                        buffer,
                    )?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            10 => {
                Ok(WorkoutStep::ExerciseCategory(Field {
                    raw_value:  profile::types::ExerciseCategory::decode::<T>(
                        buffer,
                    )?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            11 => {
                Ok(WorkoutStep::ExerciseName(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            12 => {
                Ok(WorkoutStep::ExerciseWeight(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  Some(100.0),
                    offset: None,
                    units:  Some("kg"),
                }))
            },
            13 => {
                Ok(WorkoutStep::WeightDisplayUnit(Field {
                    raw_value:  profile::types::FitBaseUnit::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            _ => {
                Ok(WorkoutStep::Unknown {
                    data: buffer.to_vec(),
                    field_def_num,
                })
            },
        }
    }
}
#[derive(Debug,Clone)]
pub enum ExerciseTitle {
    MessageIndex(Field<profile::types::MessageIndex>),
    ExerciseCategory(Field<profile::types::ExerciseCategory>),
    ExerciseName(Field<profile::base::Uint16>),
    WktStepName(Field<profile::base::Utf8String>),
    Unknown { data:          Vec<u8>, field_def_num: u8 },
}
impl ExerciseTitle {
    pub(crate) fn decode<T: ByteOrder>(
        buffer: &[u8],
        field_def_num: u8,
    ) -> error::Result<Self> {
        match field_def_num {
            254 => {
                Ok(ExerciseTitle::MessageIndex(Field {
                    raw_value:  profile::types::MessageIndex::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            0 => {
                Ok(ExerciseTitle::ExerciseCategory(Field {
                    raw_value:  profile::types::ExerciseCategory::decode::<T>(
                        buffer,
                    )?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            1 => {
                Ok(ExerciseTitle::ExerciseName(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            2 => {
                Ok(ExerciseTitle::WktStepName(Field {
                    raw_value:  profile::base::Utf8String::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            _ => {
                Ok(ExerciseTitle::Unknown {
                    data: buffer.to_vec(),
                    field_def_num,
                })
            },
        }
    }
}
#[derive(Debug,Clone)]
pub enum Schedule {
    #[doc = "Corresponds to file_id of scheduled workout / course."]
    Manufacturer(Field<profile::types::Manufacturer>),
    #[doc = "Corresponds to file_id of scheduled workout / course."]
    Product(Field<profile::base::Uint16>),
    #[doc = "Corresponds to file_id of scheduled workout / course."]
    SerialNumber(Field<profile::base::Uint32z>),
    #[doc = "Corresponds to file_id of scheduled workout / course."]
    TimeCreated(Field<profile::types::DateTime>),
    #[doc = "TRUE if this activity has been started"]
    Completed(Field<profile::base::Bool>),
    Type(Field<profile::types::Schedule>),
    ScheduledTime(Field<profile::types::LocalDateTime>),
    Unknown {
        data:          Vec<u8>,
        field_def_num: u8,
    },
}
impl Schedule {
    pub(crate) fn decode<T: ByteOrder>(
        buffer: &[u8],
        field_def_num: u8,
    ) -> error::Result<Self> {
        match field_def_num {
            0 => {
                Ok(Schedule::Manufacturer(Field {
                    raw_value:  profile::types::Manufacturer::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            1 => {
                Ok(Schedule::Product(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            2 => {
                Ok(Schedule::SerialNumber(Field {
                    raw_value:  profile::base::Uint32z::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            3 => {
                Ok(Schedule::TimeCreated(Field {
                    raw_value:  profile::types::DateTime::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            4 => {
                Ok(Schedule::Completed(Field {
                    raw_value:  profile::base::Bool::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            5 => {
                Ok(Schedule::Type(Field {
                    raw_value:  profile::types::Schedule::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            6 => {
                Ok(Schedule::ScheduledTime(Field {
                    raw_value:  profile::types::LocalDateTime::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            _ => {
                Ok(Schedule::Unknown {
                    data: buffer.to_vec(),
                    field_def_num,
                })
            },
        }
    }
}
#[derive(Debug,Clone)]
pub enum Totals {
    MessageIndex(Field<profile::types::MessageIndex>),
    Timestamp(Field<profile::types::DateTime>),
    #[doc = "Excludes pauses"]
    TimerTime(Field<profile::base::Uint32>),
    Distance(Field<profile::base::Uint32>),
    Calories(Field<profile::base::Uint32>),
    Sport(Field<profile::types::Sport>),
    #[doc = "Includes pauses"]
    ElapsedTime(Field<profile::base::Uint32>),
    Sessions(Field<profile::base::Uint16>),
    ActiveTime(Field<profile::base::Uint32>),
    SportIndex(Field<profile::base::Uint8>),
    Unknown {
        data:          Vec<u8>,
        field_def_num: u8,
    },
}
impl Totals {
    pub(crate) fn decode<T: ByteOrder>(
        buffer: &[u8],
        field_def_num: u8,
    ) -> error::Result<Self> {
        match field_def_num {
            254 => {
                Ok(Totals::MessageIndex(Field {
                    raw_value:  profile::types::MessageIndex::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            253 => {
                Ok(Totals::Timestamp(Field {
                    raw_value:  profile::types::DateTime::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("s"),
                }))
            },
            0 => {
                Ok(Totals::TimerTime(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("s"),
                }))
            },
            1 => {
                Ok(Totals::Distance(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("m"),
                }))
            },
            2 => {
                Ok(Totals::Calories(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("kcal"),
                }))
            },
            3 => {
                Ok(Totals::Sport(Field {
                    raw_value:  profile::types::Sport::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            4 => {
                Ok(Totals::ElapsedTime(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("s"),
                }))
            },
            5 => {
                Ok(Totals::Sessions(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            6 => {
                Ok(Totals::ActiveTime(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("s"),
                }))
            },
            9 => {
                Ok(Totals::SportIndex(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            _ => {
                Ok(Totals::Unknown {
                    data: buffer.to_vec(),
                    field_def_num,
                })
            },
        }
    }
}
#[derive(Debug,Clone)]
pub enum WeightScale {
    Timestamp(Field<profile::types::DateTime>),
    Weight(Field<profile::types::Weight>),
    PercentFat(Field<profile::base::Uint16>),
    PercentHydration(Field<profile::base::Uint16>),
    VisceralFatMass(Field<profile::base::Uint16>),
    BoneMass(Field<profile::base::Uint16>),
    MuscleMass(Field<profile::base::Uint16>),
    BasalMet(Field<profile::base::Uint16>),
    PhysiqueRating(Field<profile::base::Uint8>),
    #[doc = "~4kJ per kcal, 0.25 allows max 16384 kcal"]
    ActiveMet(Field<profile::base::Uint16>),
    MetabolicAge(Field<profile::base::Uint8>),
    VisceralFatRating(Field<profile::base::Uint8>),
    #[doc = "Associates this weight scale message to a user.  This \
             corresponds to the index of the user profile message in the \
             weight scale file."]
    UserProfileIndex(Field<profile::types::MessageIndex>),
    Unknown {
        data:          Vec<u8>,
        field_def_num: u8,
    },
}
impl WeightScale {
    pub(crate) fn decode<T: ByteOrder>(
        buffer: &[u8],
        field_def_num: u8,
    ) -> error::Result<Self> {
        match field_def_num {
            253 => {
                Ok(WeightScale::Timestamp(Field {
                    raw_value:  profile::types::DateTime::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("s"),
                }))
            },
            0 => {
                Ok(WeightScale::Weight(Field {
                    raw_value:  profile::types::Weight::decode::<T>(buffer)?,
                    scale:  Some(100.0),
                    offset: None,
                    units:  Some("kg"),
                }))
            },
            1 => {
                Ok(WeightScale::PercentFat(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  Some(100.0),
                    offset: None,
                    units:  Some("%"),
                }))
            },
            2 => {
                Ok(WeightScale::PercentHydration(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  Some(100.0),
                    offset: None,
                    units:  Some("%"),
                }))
            },
            3 => {
                Ok(WeightScale::VisceralFatMass(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  Some(100.0),
                    offset: None,
                    units:  Some("kg"),
                }))
            },
            4 => {
                Ok(WeightScale::BoneMass(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  Some(100.0),
                    offset: None,
                    units:  Some("kg"),
                }))
            },
            5 => {
                Ok(WeightScale::MuscleMass(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  Some(100.0),
                    offset: None,
                    units:  Some("kg"),
                }))
            },
            7 => {
                Ok(WeightScale::BasalMet(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  Some(4.0),
                    offset: None,
                    units:  Some("kcal/day"),
                }))
            },
            8 => {
                Ok(WeightScale::PhysiqueRating(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            9 => {
                Ok(WeightScale::ActiveMet(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  Some(4.0),
                    offset: None,
                    units:  Some("kcal/day"),
                }))
            },
            10 => {
                Ok(WeightScale::MetabolicAge(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("years"),
                }))
            },
            11 => {
                Ok(WeightScale::VisceralFatRating(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            12 => {
                Ok(WeightScale::UserProfileIndex(Field {
                    raw_value:  profile::types::MessageIndex::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            _ => {
                Ok(WeightScale::Unknown {
                    data: buffer.to_vec(),
                    field_def_num,
                })
            },
        }
    }
}
#[derive(Debug,Clone)]
pub enum BloodPressure {
    Timestamp(Field<profile::types::DateTime>),
    SystolicPressure(Field<profile::base::Uint16>),
    DiastolicPressure(Field<profile::base::Uint16>),
    MeanArterialPressure(Field<profile::base::Uint16>),
    Map3SampleMean(Field<profile::base::Uint16>),
    MapMorningValues(Field<profile::base::Uint16>),
    MapEveningValues(Field<profile::base::Uint16>),
    HeartRate(Field<profile::base::Uint8>),
    HeartRateType(Field<profile::types::HrType>),
    Status(Field<profile::types::BpStatus>),
    #[doc = "Associates this blood pressure message to a user.  This \
             corresponds to the index of the user profile message in the \
             blood pressure file."]
    UserProfileIndex(Field<profile::types::MessageIndex>),
    Unknown {
        data:          Vec<u8>,
        field_def_num: u8,
    },
}
impl BloodPressure {
    pub(crate) fn decode<T: ByteOrder>(
        buffer: &[u8],
        field_def_num: u8,
    ) -> error::Result<Self> {
        match field_def_num {
            253 => {
                Ok(BloodPressure::Timestamp(Field {
                    raw_value:  profile::types::DateTime::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("s"),
                }))
            },
            0 => {
                Ok(BloodPressure::SystolicPressure(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("mmHg"),
                }))
            },
            1 => {
                Ok(BloodPressure::DiastolicPressure(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("mmHg"),
                }))
            },
            2 => {
                Ok(BloodPressure::MeanArterialPressure(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("mmHg"),
                }))
            },
            3 => {
                Ok(BloodPressure::Map3SampleMean(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("mmHg"),
                }))
            },
            4 => {
                Ok(BloodPressure::MapMorningValues(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("mmHg"),
                }))
            },
            5 => {
                Ok(BloodPressure::MapEveningValues(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("mmHg"),
                }))
            },
            6 => {
                Ok(BloodPressure::HeartRate(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("bpm"),
                }))
            },
            7 => {
                Ok(BloodPressure::HeartRateType(Field {
                    raw_value:  profile::types::HrType::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            8 => {
                Ok(BloodPressure::Status(Field {
                    raw_value:  profile::types::BpStatus::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            9 => {
                Ok(BloodPressure::UserProfileIndex(Field {
                    raw_value:  profile::types::MessageIndex::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            _ => {
                Ok(BloodPressure::Unknown {
                    data: buffer.to_vec(),
                    field_def_num,
                })
            },
        }
    }
}
#[derive(Debug,Clone)]
pub enum MonitoringInfo {
    Timestamp(Field<profile::types::DateTime>),
    #[doc = "Use to convert activity timestamps to local time if device does \
             not support time zone and daylight savings time correction."]
    LocalTimestamp(Field<profile::types::LocalDateTime>),
    ActivityType(Field<profile::types::ActivityType>),
    #[doc = "Indexed by activity_type"]
    CyclesToDistance(Field<profile::base::Uint16>),
    #[doc = "Indexed by activity_type"]
    CyclesToCalories(Field<profile::base::Uint16>),
    RestingMetabolicRate(Field<profile::base::Uint16>),
    Unknown {
        data:          Vec<u8>,
        field_def_num: u8,
    },
}
impl MonitoringInfo {
    pub(crate) fn decode<T: ByteOrder>(
        buffer: &[u8],
        field_def_num: u8,
    ) -> error::Result<Self> {
        match field_def_num {
            253 => {
                Ok(MonitoringInfo::Timestamp(Field {
                    raw_value:  profile::types::DateTime::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("s"),
                }))
            },
            0 => {
                Ok(MonitoringInfo::LocalTimestamp(Field {
                    raw_value:  profile::types::LocalDateTime::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("s"),
                }))
            },
            1 => {
                Ok(MonitoringInfo::ActivityType(Field {
                    raw_value:  profile::types::ActivityType::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            3 => {
                Ok(MonitoringInfo::CyclesToDistance(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  Some(5000.0),
                    offset: None,
                    units:  Some("m/cycle"),
                }))
            },
            4 => {
                Ok(MonitoringInfo::CyclesToCalories(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  Some(5000.0),
                    offset: None,
                    units:  Some("kcal/cycle"),
                }))
            },
            5 => {
                Ok(MonitoringInfo::RestingMetabolicRate(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("kcal / day"),
                }))
            },
            _ => {
                Ok(MonitoringInfo::Unknown {
                    data: buffer.to_vec(),
                    field_def_num,
                })
            },
        }
    }
}
#[derive(Debug,Clone)]
pub enum Monitoring {
    #[doc = "Must align to logging interval, for example, time must be \
             00:00:00 for daily log."]
    Timestamp(Field<profile::types::DateTime>),
    #[doc = "Associates this data to device_info message.  Not required for \
             file with single device (sensor)."]
    DeviceIndex(Field<profile::types::DeviceIndex>),
    #[doc = "Accumulated total calories.  Maintained by MonitoringReader for \
             each activity_type.  See SDK documentation"]
    Calories(Field<profile::base::Uint16>),
    #[doc = "Accumulated distance.  Maintained by MonitoringReader for each \
             activity_type.  See SDK documentation."]
    Distance(Field<profile::base::Uint32>),
    #[doc = "Accumulated cycles.  Maintained by MonitoringReader for each \
             activity_type.  See SDK documentation."]
    Cycles(Field<profile::base::Uint32>),
    ActiveTime(Field<profile::base::Uint32>),
    ActivityType(Field<profile::types::ActivityType>),
    ActivitySubtype(Field<profile::types::ActivitySubtype>),
    ActivityLevel(Field<profile::types::ActivityLevel>),
    Distance16(Field<profile::base::Uint16>),
    Cycles16(Field<profile::base::Uint16>),
    ActiveTime16(Field<profile::base::Uint16>),
    #[doc = "Must align to logging interval, for example, time must be \
             00:00:00 for daily log."]
    LocalTimestamp(Field<profile::types::LocalDateTime>),
    #[doc = "Avg temperature during the logging interval ended at timestamp"]
    Temperature(Field<profile::base::Sint16>),
    #[doc = "Min temperature during the logging interval ended at timestamp"]
    TemperatureMin(Field<profile::base::Sint16>),
    #[doc = "Max temperature during the logging interval ended at timestamp"]
    TemperatureMax(Field<profile::base::Sint16>),
    #[doc = "Indexed using minute_activity_level enum"]
    ActivityTime(Field<profile::base::Uint16>),
    ActiveCalories(Field<profile::base::Uint16>),
    #[doc = "Indicates single type / intensity for duration since last \
             monitoring message."]
    CurrentActivityTypeIntensity(Field<profile::base::Bytes>),
    TimestampMin8(Field<profile::base::Uint8>),
    Timestamp16(Field<profile::base::Uint16>),
    HeartRate(Field<profile::base::Uint8>),
    Intensity(Field<profile::base::Uint8>),
    DurationMin(Field<profile::base::Uint16>),
    Duration(Field<profile::base::Uint32>),
    Ascent(Field<profile::base::Uint32>),
    Descent(Field<profile::base::Uint32>),
    ModerateActivityMinutes(Field<profile::base::Uint16>),
    VigorousActivityMinutes(Field<profile::base::Uint16>),
    Unknown {
        data:          Vec<u8>,
        field_def_num: u8,
    },
}
impl Monitoring {
    pub(crate) fn decode<T: ByteOrder>(
        buffer: &[u8],
        field_def_num: u8,
    ) -> error::Result<Self> {
        match field_def_num {
            253 => {
                Ok(Monitoring::Timestamp(Field {
                    raw_value:  profile::types::DateTime::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("s"),
                }))
            },
            0 => {
                Ok(Monitoring::DeviceIndex(Field {
                    raw_value:  profile::types::DeviceIndex::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            1 => {
                Ok(Monitoring::Calories(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("kcal"),
                }))
            },
            2 => {
                Ok(Monitoring::Distance(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  Some(100.0),
                    offset: None,
                    units:  Some("m"),
                }))
            },
            3 => {
                Ok(Monitoring::Cycles(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  Some(2.0),
                    offset: None,
                    units:  Some("cycles"),
                }))
            },
            4 => {
                Ok(Monitoring::ActiveTime(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  Some(1000.0),
                    offset: None,
                    units:  Some("s"),
                }))
            },
            5 => {
                Ok(Monitoring::ActivityType(Field {
                    raw_value:  profile::types::ActivityType::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            6 => {
                Ok(Monitoring::ActivitySubtype(Field {
                    raw_value:  profile::types::ActivitySubtype::decode::<T>(
                        buffer,
                    )?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            7 => {
                Ok(Monitoring::ActivityLevel(Field {
                    raw_value:  profile::types::ActivityLevel::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            8 => {
                Ok(Monitoring::Distance16(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("100 * m"),
                }))
            },
            9 => {
                Ok(Monitoring::Cycles16(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("2 * cycles (steps)"),
                }))
            },
            10 => {
                Ok(Monitoring::ActiveTime16(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("s"),
                }))
            },
            11 => {
                Ok(Monitoring::LocalTimestamp(Field {
                    raw_value:  profile::types::LocalDateTime::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            12 => {
                Ok(Monitoring::Temperature(Field {
                    raw_value:  profile::base::Sint16::decode::<T>(buffer)?,
                    scale:  Some(100.0),
                    offset: None,
                    units:  Some("C"),
                }))
            },
            14 => {
                Ok(Monitoring::TemperatureMin(Field {
                    raw_value:  profile::base::Sint16::decode::<T>(buffer)?,
                    scale:  Some(100.0),
                    offset: None,
                    units:  Some("C"),
                }))
            },
            15 => {
                Ok(Monitoring::TemperatureMax(Field {
                    raw_value:  profile::base::Sint16::decode::<T>(buffer)?,
                    scale:  Some(100.0),
                    offset: None,
                    units:  Some("C"),
                }))
            },
            16 => {
                Ok(Monitoring::ActivityTime(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("minutes"),
                }))
            },
            19 => {
                Ok(Monitoring::ActiveCalories(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("kcal"),
                }))
            },
            24 => {
                Ok(Monitoring::CurrentActivityTypeIntensity(Field {
                    raw_value:  profile::base::Bytes::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            25 => {
                Ok(Monitoring::TimestampMin8(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("min"),
                }))
            },
            26 => {
                Ok(Monitoring::Timestamp16(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("s"),
                }))
            },
            27 => {
                Ok(Monitoring::HeartRate(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("bpm"),
                }))
            },
            28 => {
                Ok(Monitoring::Intensity(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  Some(10.0),
                    offset: None,
                    units:  None,
                }))
            },
            29 => {
                Ok(Monitoring::DurationMin(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("min"),
                }))
            },
            30 => {
                Ok(Monitoring::Duration(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("s"),
                }))
            },
            31 => {
                Ok(Monitoring::Ascent(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  Some(1000.0),
                    offset: None,
                    units:  Some("m"),
                }))
            },
            32 => {
                Ok(Monitoring::Descent(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  Some(1000.0),
                    offset: None,
                    units:  Some("m"),
                }))
            },
            33 => {
                Ok(Monitoring::ModerateActivityMinutes(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("minutes"),
                }))
            },
            34 => {
                Ok(Monitoring::VigorousActivityMinutes(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("minutes"),
                }))
            },
            _ => {
                Ok(Monitoring::Unknown {
                    data: buffer.to_vec(),
                    field_def_num,
                })
            },
        }
    }
}
#[derive(Debug,Clone)]
pub enum Hr {
    Timestamp(Field<profile::types::DateTime>),
    FractionalTimestamp(Field<profile::base::Uint16>),
    Time256(Field<profile::base::Uint8>),
    FilteredBpm(Field<profile::base::Uint8>),
    EventTimestamp(Field<profile::base::Uint32>),
    EventTimestamp12(Field<profile::base::Bytes>),
    Unknown { data:          Vec<u8>, field_def_num: u8 },
}
impl Hr {
    pub(crate) fn decode<T: ByteOrder>(
        buffer: &[u8],
        field_def_num: u8,
    ) -> error::Result<Self> {
        match field_def_num {
            253 => {
                Ok(Hr::Timestamp(Field {
                    raw_value:  profile::types::DateTime::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            0 => {
                Ok(Hr::FractionalTimestamp(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  Some(32768.0),
                    offset: None,
                    units:  Some("s"),
                }))
            },
            1 => {
                Ok(Hr::Time256(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  Some(256.0),
                    offset: None,
                    units:  Some("s"),
                }))
            },
            6 => {
                Ok(Hr::FilteredBpm(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("bpm"),
                }))
            },
            9 => {
                Ok(Hr::EventTimestamp(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  Some(1024.0),
                    offset: None,
                    units:  Some("s"),
                }))
            },
            10 => {
                Ok(Hr::EventTimestamp12(Field {
                    raw_value:  profile::base::Bytes::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("s"),
                }))
            },
            _ => {
                Ok(Hr::Unknown {
                    data: buffer.to_vec(),
                    field_def_num,
                })
            },
        }
    }
}
#[doc = "Value from 1 to 100 calculated by FirstBeat"]
#[derive(Debug,Clone)]
pub enum StressLevel {
    StressLevelValue(Field<profile::base::Sint16>),
    #[doc = "Time stress score was calculated"]
    StressLevelTime(Field<profile::types::DateTime>),
    Unknown {
        data:          Vec<u8>,
        field_def_num: u8,
    },
}
impl StressLevel {
    pub(crate) fn decode<T: ByteOrder>(
        buffer: &[u8],
        field_def_num: u8,
    ) -> error::Result<Self> {
        match field_def_num {
            0 => {
                Ok(StressLevel::StressLevelValue(Field {
                    raw_value:  profile::base::Sint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            1 => {
                Ok(StressLevel::StressLevelTime(Field {
                    raw_value:  profile::types::DateTime::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("s"),
                }))
            },
            _ => {
                Ok(StressLevel::Unknown {
                    data: buffer.to_vec(),
                    field_def_num,
                })
            },
        }
    }
}
#[derive(Debug,Clone)]
pub enum MemoGlob {
    #[doc = "Sequence number of memo blocks"]
    PartIndex(Field<profile::base::Uint32>),
    #[doc = "Block of utf8 bytes"]
    Memo(Field<profile::base::Bytes>),
    #[doc = "Allows relating glob to another mesg  If used only required for \
             first part of each memo_glob"]
    MessageNumber(Field<profile::base::Uint16>),
    #[doc = "Index of external mesg"]
    MessageIndex(Field<profile::types::MessageIndex>),
    Unknown {
        data:          Vec<u8>,
        field_def_num: u8,
    },
}
impl MemoGlob {
    pub(crate) fn decode<T: ByteOrder>(
        buffer: &[u8],
        field_def_num: u8,
    ) -> error::Result<Self> {
        match field_def_num {
            250 => {
                Ok(MemoGlob::PartIndex(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            0 => {
                Ok(MemoGlob::Memo(Field {
                    raw_value:  profile::base::Bytes::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            1 => {
                Ok(MemoGlob::MessageNumber(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            2 => {
                Ok(MemoGlob::MessageIndex(Field {
                    raw_value:  profile::types::MessageIndex::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            _ => {
                Ok(MemoGlob::Unknown {
                    data: buffer.to_vec(),
                    field_def_num,
                })
            },
        }
    }
}
#[derive(Debug,Clone)]
pub enum AntChannelId {
    ChannelNumber(Field<profile::base::Uint8>),
    DeviceType(Field<profile::base::Uint8z>),
    DeviceNumber(Field<profile::base::Uint16z>),
    TransmissionType(Field<profile::base::Uint8z>),
    DeviceIndex(Field<profile::types::DeviceIndex>),
    Unknown { data:          Vec<u8>, field_def_num: u8 },
}
impl AntChannelId {
    pub(crate) fn decode<T: ByteOrder>(
        buffer: &[u8],
        field_def_num: u8,
    ) -> error::Result<Self> {
        match field_def_num {
            0 => {
                Ok(AntChannelId::ChannelNumber(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            1 => {
                Ok(AntChannelId::DeviceType(Field {
                    raw_value:  profile::base::Uint8z::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            2 => {
                Ok(AntChannelId::DeviceNumber(Field {
                    raw_value:  profile::base::Uint16z::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            3 => {
                Ok(AntChannelId::TransmissionType(Field {
                    raw_value:  profile::base::Uint8z::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            4 => {
                Ok(AntChannelId::DeviceIndex(Field {
                    raw_value:  profile::types::DeviceIndex::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            _ => {
                Ok(AntChannelId::Unknown {
                    data: buffer.to_vec(),
                    field_def_num,
                })
            },
        }
    }
}
#[derive(Debug,Clone)]
pub enum AntRx {
    Timestamp(Field<profile::types::DateTime>),
    FractionalTimestamp(Field<profile::base::Uint16>),
    MesgId(Field<profile::base::Bytes>),
    MesgData(Field<profile::base::Bytes>),
    ChannelNumber(Field<profile::base::Uint8>),
    Data(Field<profile::base::Bytes>),
    Unknown { data:          Vec<u8>, field_def_num: u8 },
}
impl AntRx {
    pub(crate) fn decode<T: ByteOrder>(
        buffer: &[u8],
        field_def_num: u8,
    ) -> error::Result<Self> {
        match field_def_num {
            253 => {
                Ok(AntRx::Timestamp(Field {
                    raw_value:  profile::types::DateTime::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("s"),
                }))
            },
            0 => {
                Ok(AntRx::FractionalTimestamp(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  Some(32768.0),
                    offset: None,
                    units:  Some("s"),
                }))
            },
            1 => {
                Ok(AntRx::MesgId(Field {
                    raw_value:  profile::base::Bytes::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            2 => {
                Ok(AntRx::MesgData(Field {
                    raw_value:  profile::base::Bytes::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            3 => {
                Ok(AntRx::ChannelNumber(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            4 => {
                Ok(AntRx::Data(Field {
                    raw_value:  profile::base::Bytes::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            _ => {
                Ok(AntRx::Unknown {
                    data: buffer.to_vec(),
                    field_def_num,
                })
            },
        }
    }
}
#[derive(Debug,Clone)]
pub enum AntTx {
    Timestamp(Field<profile::types::DateTime>),
    FractionalTimestamp(Field<profile::base::Uint16>),
    MesgId(Field<profile::base::Bytes>),
    MesgData(Field<profile::base::Bytes>),
    ChannelNumber(Field<profile::base::Uint8>),
    Data(Field<profile::base::Bytes>),
    Unknown { data:          Vec<u8>, field_def_num: u8 },
}
impl AntTx {
    pub(crate) fn decode<T: ByteOrder>(
        buffer: &[u8],
        field_def_num: u8,
    ) -> error::Result<Self> {
        match field_def_num {
            253 => {
                Ok(AntTx::Timestamp(Field {
                    raw_value:  profile::types::DateTime::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("s"),
                }))
            },
            0 => {
                Ok(AntTx::FractionalTimestamp(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  Some(32768.0),
                    offset: None,
                    units:  Some("s"),
                }))
            },
            1 => {
                Ok(AntTx::MesgId(Field {
                    raw_value:  profile::base::Bytes::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            2 => {
                Ok(AntTx::MesgData(Field {
                    raw_value:  profile::base::Bytes::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            3 => {
                Ok(AntTx::ChannelNumber(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            4 => {
                Ok(AntTx::Data(Field {
                    raw_value:  profile::base::Bytes::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            _ => {
                Ok(AntTx::Unknown {
                    data: buffer.to_vec(),
                    field_def_num,
                })
            },
        }
    }
}
#[derive(Debug,Clone)]
pub enum ExdScreenConfiguration {
    ScreenIndex(Field<profile::base::Uint8>),
    #[doc = "number of fields in screen"]
    FieldCount(Field<profile::base::Uint8>),
    Layout(Field<profile::types::ExdLayout>),
    ScreenEnabled(Field<profile::base::Bool>),
    Unknown {
        data:          Vec<u8>,
        field_def_num: u8,
    },
}
impl ExdScreenConfiguration {
    pub(crate) fn decode<T: ByteOrder>(
        buffer: &[u8],
        field_def_num: u8,
    ) -> error::Result<Self> {
        match field_def_num {
            0 => {
                Ok(ExdScreenConfiguration::ScreenIndex(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            1 => {
                Ok(ExdScreenConfiguration::FieldCount(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            2 => {
                Ok(ExdScreenConfiguration::Layout(Field {
                    raw_value:  profile::types::ExdLayout::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            3 => {
                Ok(ExdScreenConfiguration::ScreenEnabled(Field {
                    raw_value:  profile::base::Bool::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            _ => {
                Ok(ExdScreenConfiguration::Unknown {
                    data: buffer.to_vec(),
                    field_def_num,
                })
            },
        }
    }
}
#[derive(Debug,Clone)]
pub enum ExdDataFieldConfiguration {
    ScreenIndex(Field<profile::base::Uint8>),
    ConceptField(Field<profile::base::Bytes>),
    FieldId(Field<profile::base::Uint8>),
    ConceptCount(Field<profile::base::Uint8>),
    DisplayType(Field<profile::types::ExdDisplayType>),
    Title(Field<profile::base::Utf8String>),
    Unknown { data:          Vec<u8>, field_def_num: u8 },
}
impl ExdDataFieldConfiguration {
    pub(crate) fn decode<T: ByteOrder>(
        buffer: &[u8],
        field_def_num: u8,
    ) -> error::Result<Self> {
        match field_def_num {
            0 => {
                Ok(ExdDataFieldConfiguration::ScreenIndex(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            1 => {
                Ok(ExdDataFieldConfiguration::ConceptField(Field {
                    raw_value:  profile::base::Bytes::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            2 => {
                Ok(ExdDataFieldConfiguration::FieldId(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            3 => {
                Ok(ExdDataFieldConfiguration::ConceptCount(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            4 => {
                Ok(ExdDataFieldConfiguration::DisplayType(Field {
                    raw_value:  profile::types::ExdDisplayType::decode::<T>(
                        buffer,
                    )?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            5 => {
                Ok(ExdDataFieldConfiguration::Title(Field {
                    raw_value:  profile::base::Utf8String::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            _ => {
                Ok(ExdDataFieldConfiguration::Unknown {
                    data: buffer.to_vec(),
                    field_def_num,
                })
            },
        }
    }
}
#[derive(Debug,Clone)]
pub enum ExdDataConceptConfiguration {
    ScreenIndex(Field<profile::base::Uint8>),
    ConceptField(Field<profile::base::Bytes>),
    FieldId(Field<profile::base::Uint8>),
    ConceptIndex(Field<profile::base::Uint8>),
    DataPage(Field<profile::base::Uint8>),
    ConceptKey(Field<profile::base::Uint8>),
    Scaling(Field<profile::base::Uint8>),
    DataUnits(Field<profile::types::ExdDataUnits>),
    Qualifier(Field<profile::types::ExdQualifiers>),
    Descriptor(Field<profile::types::ExdDescriptors>),
    IsSigned(Field<profile::base::Bool>),
    Unknown { data:          Vec<u8>, field_def_num: u8 },
}
impl ExdDataConceptConfiguration {
    pub(crate) fn decode<T: ByteOrder>(
        buffer: &[u8],
        field_def_num: u8,
    ) -> error::Result<Self> {
        match field_def_num {
            0 => {
                Ok(ExdDataConceptConfiguration::ScreenIndex(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            1 => {
                Ok(ExdDataConceptConfiguration::ConceptField(Field {
                    raw_value:  profile::base::Bytes::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            2 => {
                Ok(ExdDataConceptConfiguration::FieldId(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            3 => {
                Ok(ExdDataConceptConfiguration::ConceptIndex(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            4 => {
                Ok(ExdDataConceptConfiguration::DataPage(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            5 => {
                Ok(ExdDataConceptConfiguration::ConceptKey(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            6 => {
                Ok(ExdDataConceptConfiguration::Scaling(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            8 => {
                Ok(ExdDataConceptConfiguration::DataUnits(Field {
                    raw_value:  profile::types::ExdDataUnits::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            9 => {
                Ok(ExdDataConceptConfiguration::Qualifier(Field {
                    raw_value:  profile::types::ExdQualifiers::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            10 => {
                Ok(ExdDataConceptConfiguration::Descriptor(Field {
                    raw_value:  profile::types::ExdDescriptors::decode::<T>(
                        buffer,
                    )?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            11 => {
                Ok(ExdDataConceptConfiguration::IsSigned(Field {
                    raw_value:  profile::base::Bool::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            _ => {
                Ok(ExdDataConceptConfiguration::Unknown {
                    data: buffer.to_vec(),
                    field_def_num,
                })
            },
        }
    }
}
#[doc = "Must be logged before developer field is used"]
#[derive(Debug,Clone)]
pub enum FieldDescription {
    DeveloperDataIndex(Field<profile::base::Uint8>),
    FieldDefinitionNumber(Field<profile::base::Uint8>),
    FitBaseTypeId(Field<profile::types::FitBaseType>),
    FieldName(Field<profile::base::Utf8String>),
    Array(Field<profile::base::Uint8>),
    Components(Field<profile::base::Utf8String>),
    Scale(Field<profile::base::Uint8>),
    Offset(Field<profile::base::Sint8>),
    Units(Field<profile::base::Utf8String>),
    Bits(Field<profile::base::Utf8String>),
    Accumulate(Field<profile::base::Utf8String>),
    FitBaseUnitId(Field<profile::types::FitBaseUnit>),
    NativeMesgNum(Field<profile::types::MesgNum>),
    NativeFieldNum(Field<profile::base::Uint8>),
    Unknown { data:          Vec<u8>, field_def_num: u8 },
}
impl FieldDescription {
    pub(crate) fn decode<T: ByteOrder>(
        buffer: &[u8],
        field_def_num: u8,
    ) -> error::Result<Self> {
        match field_def_num {
            0 => {
                Ok(FieldDescription::DeveloperDataIndex(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            1 => {
                Ok(FieldDescription::FieldDefinitionNumber(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            2 => {
                Ok(FieldDescription::FitBaseTypeId(Field {
                    raw_value:  profile::types::FitBaseType::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            3 => {
                Ok(FieldDescription::FieldName(Field {
                    raw_value:  profile::base::Utf8String::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            4 => {
                Ok(FieldDescription::Array(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            5 => {
                Ok(FieldDescription::Components(Field {
                    raw_value:  profile::base::Utf8String::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            6 => {
                Ok(FieldDescription::Scale(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            7 => {
                Ok(FieldDescription::Offset(Field {
                    raw_value:  profile::base::Sint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            8 => {
                Ok(FieldDescription::Units(Field {
                    raw_value:  profile::base::Utf8String::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            9 => {
                Ok(FieldDescription::Bits(Field {
                    raw_value:  profile::base::Utf8String::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            10 => {
                Ok(FieldDescription::Accumulate(Field {
                    raw_value:  profile::base::Utf8String::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            13 => {
                Ok(FieldDescription::FitBaseUnitId(Field {
                    raw_value:  profile::types::FitBaseUnit::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            14 => {
                Ok(FieldDescription::NativeMesgNum(Field {
                    raw_value:  profile::types::MesgNum::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            15 => {
                Ok(FieldDescription::NativeFieldNum(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            _ => {
                Ok(FieldDescription::Unknown {
                    data: buffer.to_vec(),
                    field_def_num,
                })
            },
        }
    }
}
#[doc = "Must be logged before field description"]
#[derive(Debug,Clone)]
pub enum DeveloperDataId {
    DeveloperId(Field<profile::base::Bytes>),
    ApplicationId(Field<profile::base::Bytes>),
    ManufacturerId(Field<profile::types::Manufacturer>),
    DeveloperDataIndex(Field<profile::base::Uint8>),
    ApplicationVersion(Field<profile::base::Uint32>),
    Unknown { data:          Vec<u8>, field_def_num: u8 },
}
impl DeveloperDataId {
    pub(crate) fn decode<T: ByteOrder>(
        buffer: &[u8],
        field_def_num: u8,
    ) -> error::Result<Self> {
        match field_def_num {
            0 => {
                Ok(DeveloperDataId::DeveloperId(Field {
                    raw_value:  profile::base::Bytes::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            1 => {
                Ok(DeveloperDataId::ApplicationId(Field {
                    raw_value:  profile::base::Bytes::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            2 => {
                Ok(DeveloperDataId::ManufacturerId(Field {
                    raw_value:  profile::types::Manufacturer::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            3 => {
                Ok(DeveloperDataId::DeveloperDataIndex(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            4 => {
                Ok(DeveloperDataId::ApplicationVersion(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            _ => {
                Ok(DeveloperDataId::Unknown {
                    data: buffer.to_vec(),
                    field_def_num,
                })
            },
        }
    }
}
#[derive(Debug,Clone)]
pub enum DiveSummary {
    Timestamp(Field<profile::types::DateTime>),
    ReferenceMesg(Field<profile::types::MesgNum>),
    ReferenceIndex(Field<profile::types::MessageIndex>),
    #[doc = "0 if above water"]
    AvgDepth(Field<profile::base::Uint32>),
    #[doc = "0 if above water"]
    MaxDepth(Field<profile::base::Uint32>),
    #[doc = "Time since end of last dive"]
    SurfaceInterval(Field<profile::base::Uint32>),
    StartCns(Field<profile::base::Uint8>),
    EndCns(Field<profile::base::Uint8>),
    StartN2(Field<profile::base::Uint16>),
    EndN2(Field<profile::base::Uint16>),
    O2Toxicity(Field<profile::base::Uint16>),
    DiveNumber(Field<profile::base::Uint32>),
    BottomTime(Field<profile::base::Uint32>),
    Unknown {
        data:          Vec<u8>,
        field_def_num: u8,
    },
}
impl DiveSummary {
    pub(crate) fn decode<T: ByteOrder>(
        buffer: &[u8],
        field_def_num: u8,
    ) -> error::Result<Self> {
        match field_def_num {
            253 => {
                Ok(DiveSummary::Timestamp(Field {
                    raw_value:  profile::types::DateTime::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("s"),
                }))
            },
            0 => {
                Ok(DiveSummary::ReferenceMesg(Field {
                    raw_value:  profile::types::MesgNum::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            1 => {
                Ok(DiveSummary::ReferenceIndex(Field {
                    raw_value:  profile::types::MessageIndex::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            2 => {
                Ok(DiveSummary::AvgDepth(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  Some(1000.0),
                    offset: None,
                    units:  Some("m"),
                }))
            },
            3 => {
                Ok(DiveSummary::MaxDepth(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  Some(1000.0),
                    offset: None,
                    units:  Some("m"),
                }))
            },
            4 => {
                Ok(DiveSummary::SurfaceInterval(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  Some(1.0),
                    offset: None,
                    units:  Some("s"),
                }))
            },
            5 => {
                Ok(DiveSummary::StartCns(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  Some(1.0),
                    offset: None,
                    units:  Some("percent"),
                }))
            },
            6 => {
                Ok(DiveSummary::EndCns(Field {
                    raw_value:  profile::base::Uint8::decode::<T>(buffer)?,
                    scale:  Some(1.0),
                    offset: None,
                    units:  Some("percent"),
                }))
            },
            7 => {
                Ok(DiveSummary::StartN2(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  Some(1.0),
                    offset: None,
                    units:  Some("percent"),
                }))
            },
            8 => {
                Ok(DiveSummary::EndN2(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  Some(1.0),
                    offset: None,
                    units:  Some("percent"),
                }))
            },
            9 => {
                Ok(DiveSummary::O2Toxicity(Field {
                    raw_value:  profile::base::Uint16::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  Some("OTUs"),
                }))
            },
            10 => {
                Ok(DiveSummary::DiveNumber(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  None,
                    offset: None,
                    units:  None,
                }))
            },
            11 => {
                Ok(DiveSummary::BottomTime(Field {
                    raw_value:  profile::base::Uint32::decode::<T>(buffer)?,
                    scale:  Some(1000.0),
                    offset: None,
                    units:  Some("s"),
                }))
            },
            _ => {
                Ok(DiveSummary::Unknown {
                    data: buffer.to_vec(),
                    field_def_num,
                })
            },
        }
    }
}
