/// Label for DBSCAN clustering.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Label {
    Assigned(usize),
    Outlier,
    Marked,
    Undefined,
}

impl Label {
    /// Return whether the label is assigned.
    pub fn is_assigned(&self) -> bool {
        matches!(*self, Label::Assigned(_))
    }

    /// Return whether the label is outlier.
    pub fn is_outlier(&self) -> bool {
        matches!(*self, Label::Outlier)
    }

    /// Return whether the label is undefined.
    pub fn is_undefined(&self) -> bool {
        matches!(*self, Label::Undefined)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_assigned_should_return_true_if_label_is_assigned() {
        assert_eq!(Label::Assigned(0).is_assigned(), true);
        assert_eq!(Label::Outlier.is_assigned(), false);
        assert_eq!(Label::Marked.is_assigned(), false);
        assert_eq!(Label::Undefined.is_assigned(), false);
    }

    #[test]
    fn is_outlier_should_return_true_if_label_is_outlier() {
        assert_eq!(Label::Assigned(0).is_outlier(), false);
        assert_eq!(Label::Outlier.is_outlier(), true);
        assert_eq!(Label::Marked.is_outlier(), false);
        assert_eq!(Label::Undefined.is_outlier(), false);
    }

    #[test]
    fn is_undefined_should_return_true_if_label_is_undefined() {
        assert_eq!(Label::Assigned(0).is_undefined(), false);
        assert_eq!(Label::Outlier.is_undefined(), false);
        assert_eq!(Label::Marked.is_undefined(), false);
        assert_eq!(Label::Undefined.is_undefined(), true);
    }
}
