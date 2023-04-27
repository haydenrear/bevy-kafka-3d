use crate::metrics::HistoricalData;

#[test]
fn test_historical_data() {
    let mut historical_data = HistoricalData::new(3);

    historical_data.push(1.0);
    historical_data.push(2.0);
    historical_data.push(3.0);

    assert_eq!(historical_data.get(0), Some(3.0));
    assert_eq!(historical_data.get(1), Some(2.0));
    assert_eq!(historical_data.get(2), Some(1.0));
    assert_eq!(historical_data.get(3), None);

    historical_data.push(4.0);

    assert_eq!(historical_data.get(0), Some(4.0));
    assert_eq!(historical_data.get(1), Some(3.0));
    assert_eq!(historical_data.get(2), Some(2.0));
    assert_eq!(historical_data.get(3), None);

    historical_data.push(5.0);

    assert_eq!(historical_data.get(0), Some(5.0));
    assert_eq!(historical_data.get(1), Some(4.0));
    assert_eq!(historical_data.get(2), Some(3.0));
    assert_eq!(historical_data.get(3), None);
}