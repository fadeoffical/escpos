pub trait Command {
    fn get_name(&self) -> &str;
    fn get_description(&self) -> &str;

    // fn with_data(&self, data: &[u8]) -> Vec<u8>;
}

pub struct ESC;

pub mod print {
    pub struct PrintAndLineFeed;
    pub struct PrintAndReturnToStandardMode;
    pub struct EndJob;
    pub struct PrintAndCarriageReturn;
    pub struct PrintDataInPageMode;
    pub struct PrintAndFeedPaper;
    pub struct PrintAndReverseFeed;
    pub struct PrintAndFeedLines;
    pub struct PrintAndReverseFeedLines;
}

pub mod spacing {
    pub struct SelectDefaultLineSpacing;
    pub struct SetLineSpacing;
}

pub mod character {
    pub struct CancelPrintDataInPageMode;

}
