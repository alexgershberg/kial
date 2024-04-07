use crate::pear::Pear;

#[derive(Debug, PartialEq)]
pub(crate) struct Ident(pub String);

impl TryFrom<&mut Pear<'_>> for Ident {
    type Error = String;

    fn try_from(pear: &mut Pear) -> Result<Self, Self::Error> {
        let identifier = pear.extract_identifier()?.val;
        Ok(Self(identifier))
    }
}
