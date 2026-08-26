/// Sheffield Rules - All 19 Annual Rulesets (1858-1877)
///
/// This module provides all distinct rulesets that existed during the
/// Sheffield Rules period, with implementations of the RuleSet trait.

// Individual year implementations
mod y1858;
mod y1859;
mod y1860;
mod y1861;
mod y1862;
mod y1864;
mod y1863;
mod y1865;
mod y1866;
mod y1867_march;
mod y1867_october;
mod y1868;
mod y1869;
mod y1870;
mod y1871;
mod y1872;
mod y1873;
mod y1874;
mod y1875;
mod y1876;

// Re-export all rulesets
pub use y1858::Ruleset1858;
pub use y1859::Ruleset1859;
pub use y1860::Ruleset1860;
pub use y1861::Ruleset1861;
pub use y1862::Ruleset1862;
pub use y1863::Ruleset1863;
pub use y1864::Ruleset1864;
pub use y1865::Ruleset1865;
pub use y1866::Ruleset1866;
pub use y1867_march::Ruleset1867March;
pub use y1867_october::Ruleset1867October;
pub use y1868::Ruleset1868;
pub use y1869::Ruleset1869;
pub use y1870::Ruleset1870;
pub use y1871::Ruleset1871;
pub use y1872::Ruleset1872;
pub use y1873::Ruleset1873;
pub use y1874::Ruleset1874;
pub use y1875::Ruleset1875;
pub use y1876::Ruleset1876;

use crate::sheffield_rules::ruleset::RuleSet;
use std::sync::Arc;

/// Get the appropriate ruleset for a given year
///
/// # Arguments
/// * `year` - The year (1858-1877)
/// * `month` - The month (1-12), used for 1867 which had two distinct rulesets
///
/// # Returns
/// An Arc-wrapped boxed RuleSet trait object
///
/// # Panics
/// Panics if year is outside 1858-1877 range
pub fn get_ruleset_for_year(year: u32, month: Option<u32>) -> Arc<dyn RuleSet> {
    match year {
        1858 => Arc::new(Ruleset1858),
        1859 => Arc::new(Ruleset1859),
        1860 => Arc::new(Ruleset1860),
        1861 => Arc::new(Ruleset1861),
        1862 => Arc::new(Ruleset1862),
        1863 => Arc::new(Ruleset1863),
        1864 => Arc::new(Ruleset1864),
        1865 => Arc::new(Ruleset1865),
        1866 => Arc::new(Ruleset1866),
        1867 => {
            // 1867 had two distinct rulesets: March and October
            if let Some(m) = month {
                if m >= 10 {
                    Arc::new(Ruleset1867October)
                } else {
                    Arc::new(Ruleset1867March)
                }
            } else {
                // Default to March if month not specified
                Arc::new(Ruleset1867March)
            }
        }
        1868 => Arc::new(Ruleset1868),
        1869 => Arc::new(Ruleset1869),
        1870 => Arc::new(Ruleset1870),
        1871 => Arc::new(Ruleset1871),
        1872 => Arc::new(Ruleset1872),
        1873 => Arc::new(Ruleset1873),
        1874 => Arc::new(Ruleset1874),
        1875 => Arc::new(Ruleset1875),
        1876 => Arc::new(Ruleset1876),
        _ => panic!(
            "Year {} is outside Sheffield Rules period (1858-1877)",
            year
        ),
    }
}

/// List of all distinct ruleset years for menu/selection
pub fn all_ruleset_years() -> Vec<(u32, &'static str)> {
    vec![
        (1858, "1858 Sheffield Rules (Original)"),
        (1859, "1859 Sheffield Rules"),
        (1860, "1860 Sheffield Rules"),
        (1861, "1861 Sheffield Rules"),
        (1862, "1862 Sheffield Rules (Rouge Era)"),
        (1863, "1863 Sheffield Rules"),
        (1864, "1864 Sheffield Rules"),
        (1865, "1865 Sheffield Rules (Strict Offside)"),
        (1866, "1866 Sheffield Rules"),
        (1867, "1867 March Sheffield Rules (FA Rules)"),
        (1868, "1868 Sheffield Rules (Post-Rouge, Corner Kicks)"),
        (1869, "1869 Sheffield Rules (Handling Restoration)"),
        (1870, "1870 Sheffield Rules"),
        (1871, "1871 Sheffield Rules (Handling Restriction)"),
        (1872, "1872 Sheffield Rules"),
        (1873, "1873 Sheffield Rules"),
        (1874, "1874 Sheffield Rules"),
        (1875, "1875 Sheffield Rules (Stable Era)"),
        (1876, "1876 Sheffield Rules"),
    ]
}

/// Get a summary description of what changed in a given year
pub fn year_changes_summary(year: u32) -> &'static str {
    match year {
        1858 => "Original Sheffield Rules - No offside, fair catch allowed",
        1859 => "No changes from 1858",
        1860 => "Handling rules tightened",
        1861 => "No changes from 1860",
        1862 => "MAJOR: Rouge introduced (12×9 ft goals)",
        1863 => "Weak offside rule experimented",
        1864 => "No changes from 1863",
        1865 => "Strict offside attempted (failed)",
        1866 => "Offside reverted to weak rule",
        1867 => "MAJOR: Handling banned, rouge changed (no touchdown required)",
        1868 => "EXTREME: Rouge abolished, goals doubled (24×9 ft), corner kicks introduced",
        1869 => "Handling partially restored, attempted catch allowed",
        1870 => "No changes from 1869",
        1871 => "Fair catch abolished, handling restricted to 3 yards",
        1872 => "No changes from 1871",
        1873 => "No changes from 1871",
        1874 => "No changes from 1871",
        1875 => "Goal height standardized to 8 ft (24×8 ft final dimensions)",
        1876 => "Goalkeeper handling rule clarified",
        _ => "Unknown year",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_ruleset_returns_correct_year() {
        let ruleset = get_ruleset_for_year(1862, None);
        assert_eq!(ruleset.year(), 1862);
        assert_eq!(ruleset.name(), "1862 Sheffield Rules (Rouge Era)");
    }

    #[test]
    fn test_1867_month_routing() {
        let march_ruleset = get_ruleset_for_year(1867, Some(3));
        let october_ruleset = get_ruleset_for_year(1867, Some(10));

        assert_eq!(march_ruleset.name(), "1867 March Sheffield Rules (FA Rules)");
        assert_eq!(
            october_ruleset.name(),
            "1867 October Sheffield Rules"
        );
    }

    #[test]
    fn test_goal_dimensions() {
        let y1862 = get_ruleset_for_year(1862, None);
        assert_eq!(y1862.goal_width(), Some(12));
        assert_eq!(y1862.goal_height(), Some(9));

        let y1868 = get_ruleset_for_year(1868, None);
        assert_eq!(y1868.goal_width(), Some(24));
        assert_eq!(y1868.goal_height(), Some(9));

        let y1875 = get_ruleset_for_year(1875, None);
        assert_eq!(y1875.goal_width(), Some(24));
        assert_eq!(y1875.goal_height(), Some(8));
    }

    #[test]
    fn test_rouge_presence() {
        let y1862 = get_ruleset_for_year(1862, None);
        assert!(y1862.scoring_system().has_rouge());

        let y1868 = get_ruleset_for_year(1868, None);
        assert!(!y1868.scoring_system().has_rouge());

        let y1875 = get_ruleset_for_year(1875, None);
        assert!(!y1875.scoring_system().has_rouge());
    }

    #[test]
    fn test_all_ruleset_years_count() {
        let years = all_ruleset_years();
        assert_eq!(years.len(), 19);
    }

    #[test]
    #[should_panic]
    fn test_invalid_year_panics() {
        let _ = get_ruleset_for_year(1880, None);
    }
}
