struct Event {
    timestamp: u64,
    sensor: String,
    metric: SensorMetric,
    value: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum SensorMetric {
    Load,
    Frequency,
    Temperature,
}

// Имплементирайте следната много генерична помощна функция, която:
// - приема списък от събития
// - групира събитията по ключ K
// - взима данните за агрегация D, от всяко събитие, ако има такива
// - за всеки ключ K, прилага функцията aggregate върху всички данни и записва в hash map ключа и резултата от агрегацията
fn group_and_aggregate<O, E, D, K>(
    events: &[E],
    group_fn: impl Fn(&E) -> K,
    data_fn: impl Fn(&E) -> Option<D>,
    aggregate: impl Fn(&[D]) -> O,
) -> HashMap<K, O>
where
    K: Eq + Hash,
{
    todo!()
}


trait Aggregator {
    type Output;

    fn name(&self) -> String;
    fn aggregate(&self, events: &[Event]) -> Vec<(String, Self::Output)>;
}

// Имплементирайте агрегация, която връща средната стойност на метриките, групирани по тип.
// Резултата трябва да е вектор от три елемента:
// - ("Load", <средната стойност на Load метриките от всички сензори>)
// - ("Frequency", <средната стойност на Frequency метриките от всички сензори>)
// - ("Temperature", <средната стойност на Temperature метриките от всички сензори>)
struct TotalAggregator {}
impl Aggregator for TotalAggregator {
    type Output = f64;

    fn name(&self) -> String {
        "total".to_string()
    }

    fn aggregate(&self, events: &[Event]) -> Vec<(String, f64)> {
        todo!()
    }
}

// Имплементирайте агрегация, която по даден тип на метрика, връща сумата от метриките
// от този тип, групирани по сензор.
// Пример при `MetricAggreator { metric: SensorMetric::Load }`, може да върне:
// - ("cpu0", <сумата от Load метиките за сензор cpu0>)
// - ("cpu1", <сумата от Load метиките за сензор cpu1>)
// - ("gpu0", <сумата от Load метиките за сензор gpu0>)
struct MetricAggregator {
    metric: SensorMetric
}
impl Aggregator for MetricAggregator {
    type Output = f64;

    fn name(&self) -> String {
        format!("{:?}", self.metric);
    }

    fn aggregate(&self, events: &[Event]) -> Vec<(String, f64)> {
        todo!();
    }
}

// Пример за изпозлване
fn main() {
    let _all: &[Box<dyn Aggregator<Output=f64>>] = &[
        Box::new(TotalAggregator{}),
        Box::new(MetricAggregator{metric: SensorMetric::Load}),
        Box::new(MetricAggregator{metric: SensorMetric::Temperature}),
    ];

    let _events: &[Event] = &[];

    // ...
}