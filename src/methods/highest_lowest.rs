use crate::core::Method;
use crate::core::{PeriodType, ValueType, Window};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Calculates absolute difference between highest and lowest values over the last `length` values for timeseries of type [`ValueType`]
///
/// # Parameters
///
/// Has a single parameter `length`: [`PeriodType`]
///
/// `length` should be > 0
///
/// # Input type
///
/// Input type is [`ValueType`]
///
/// # Output type
///
/// Output type is [`ValueType`]
///
/// Output value is always >= 0.0
///
/// # Examples
///
/// ```
/// use yata::prelude::*;
/// use yata::methods::HighestLowestDelta;
///
///
/// let values = [1.0, 2.0, 3.0, 2.0, 1.0, 0.5, 2.0, 3.0];
/// let r      = [0.0, 1.0, 2.0, 1.0, 2.0, 1.5, 1.5, 2.5];
/// let mut hld = HighestLowestDelta::new(3, values[0]);
///
/// (0..values.len()).for_each(|i| {
/// 	let v = hld.next(values[i]);
/// 	assert_eq!(v, r[i]);
/// });
/// ```
///
/// # Perfomance
///
/// O(`length`)
///
/// This method is relatively very slow compare to the other methods.
///
/// # See also
///
/// [Highest], [Lowest]
///
/// [`ValueType`]: crate::core::ValueType
/// [`PeriodType`]: crate::core::PeriodType
#[derive(Debug, Default, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct HighestLowestDelta {
	highest: Highest,
	lowest: Lowest,
}

impl Method for HighestLowestDelta {
	type Params = PeriodType;
	type Input = ValueType;
	type Output = Self::Input;

	fn new(length: Self::Params, value: Self::Input) -> Self
	where
		Self: Sized,
	{
		debug_assert!(length > 0, "HighestLowestDelta: length should be > 0");

		Self {
			highest: Highest::new(length, value),
			lowest: Lowest::new(length, value),
		}
	}

	#[inline]
	fn next(&mut self, value: ValueType) -> ValueType {
		self.highest.next(value) - self.lowest.next(value)
	}
}

/// Returns highest value over the last `length` values for timeseries of type [`ValueType`]
///
/// # Parameters
///
/// Has a single parameter `length`: [`PeriodType`]
///
/// `length` should be > 0
///
/// # Input type
///
/// Input type is [`ValueType`]
///
/// # Output type
///
/// Output type is [`ValueType`]
///
/// # Examples
///
/// ```
/// use yata::core::Method;
/// use yata::methods::Highest;
///
/// let values = [1.0, 2.0, 3.0, 2.0, 1.0, 0.5, 2.0, 3.0];
/// let r      = [1.0, 2.0, 3.0, 3.0, 3.0, 2.0, 2.0, 3.0];
///
/// let mut highest = Highest::new(3, values[0]);
///
/// (0..values.len()).for_each(|i| {
/// 	let v = highest.next(values[i]);
/// 	assert_eq!(v, r[i]);
/// });
/// ```
///
/// # Perfomance
///
/// O(`length`)
///
/// This method is relatively slow compare to the other methods.
///
/// # See also
///
/// [HighestLowestDelta], [Lowest]
///
/// [`ValueType`]: crate::core::ValueType
/// [`PeriodType`]: crate::core::PeriodType
#[derive(Debug, Default, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Highest {
	value: ValueType,
	window: Window<ValueType>,
}

impl Method for Highest {
	type Params = PeriodType;
	type Input = ValueType;
	type Output = Self::Input;

	fn new(length: Self::Params, value: Self::Input) -> Self {
		debug_assert!(length > 0, "Highest: length should be > 0");

		Self {
			window: Window::new(length, value),
			value,
		}
	}

	#[inline]
	fn next(&mut self, value: ValueType) -> ValueType {
		self.window.push(value);

		if value > self.value {
			self.value = value;
		} else {
			self.value = self.window.iter().fold(value, |a, b| a.max(b));
		}

		self.value
	}
}

/// Returns lowest value over the last `length` values for timeseries of type [`ValueType`]
///
/// # Parameters
///
/// Has a single parameter `length`: [`PeriodType`]
///
/// `length` should be > 0
///
/// # Input type
///
/// Input type is [`ValueType`]
///
/// # Output type
///
/// Output type is [`ValueType`]
///
/// # Examples
///
/// ```
/// use yata::core::Method;
/// use yata::methods::Lowest;
///
/// let values = [1.0, 2.0, 3.0, 2.0, 1.0, 0.5, 2.0, 3.0];
/// let r      = [1.0, 1.0, 1.0, 2.0, 1.0, 0.5, 0.5, 0.5];
///
/// let mut lowest = Lowest::new(3, values[0]);
///
/// (0..values.len()).for_each(|i| {
/// 	let v = lowest.next(values[i]);
/// 	assert_eq!(v, r[i]);
/// });
/// ```
///
/// # Perfomance
///
/// O(`length`)
///
/// This method is relatively slow compare to the other methods.
///
/// # See also
///
/// [HighestLowestDelta], [Highest]
///
/// [`ValueType`]: crate::core::ValueType
/// [`PeriodType`]: crate::core::PeriodType
#[derive(Debug, Default, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Lowest {
	value: ValueType,
	window: Window<ValueType>,
}

impl Method for Lowest {
	type Params = PeriodType;
	type Input = ValueType;
	type Output = Self::Input;

	fn new(length: Self::Params, value: Self::Input) -> Self {
		debug_assert!(length > 0, "Lowest: length should be > 0");

		Self {
			window: Window::new(length, value),
			value,
		}
	}

	#[inline]
	fn next(&mut self, value: ValueType) -> ValueType {
		self.window.push(value);

		if value < self.value {
			self.value = value;
		} else {
			self.value = self.window.iter().fold(value, |a, b| a.min(b));
		}

		self.value
	}
}

#[cfg(test)]
mod tests {
	#![allow(unused_imports)]
	use crate::core::{PeriodType, ValueType};
	use crate::helpers::RandomCandles;

	#[allow(dead_code)]
	const SIGMA: ValueType = 1e-8;

	#[test]
	fn test_highest_const() {
		use super::*;
		use crate::core::{Candle, Method};
		use crate::methods::tests::test_const;

		for i in 1..30 {
			let input = (i as ValueType + 56.0) / 16.3251;
			let mut method = Highest::new(i, input);

			let output = method.next(input);
			test_const(&mut method, input, output);
		}
	}

	#[test]
	fn test_highest1() {
		use super::{Highest as TestingMethod, Method};

		let mut candles = RandomCandles::default();

		let mut ma = TestingMethod::new(1, candles.first().close);

		candles.take(100).for_each(|x| {
			assert_eq!(x.close, ma.next(x.close));
		});
	}

	#[test]
	fn test_highest() {
		use super::{Highest as TestingMethod, Method};

		let candles = RandomCandles::default();

		let src: Vec<ValueType> = candles.take(100).map(|x| x.close).collect();

		(2..20).for_each(|length| {
			let mut ma = TestingMethod::new(length, src[0]);
			let length = length as usize;

			src.iter().enumerate().for_each(|(i, &x)| {
				let value1 = ma.next(x);
				let value2 = (0..length).fold(src[i], |m, j| m.max(src[i.saturating_sub(j)]));
				assert_eq!(value2, value1);
			});
		});
	}

	#[test]
	fn test_lowest_const() {
		use super::*;
		use crate::core::{Candle, Method};
		use crate::methods::tests::test_const;

		for i in 1..30 {
			let input = (i as ValueType + 56.0) / 16.3251;
			let mut method = Lowest::new(i, input);

			let output = method.next(input);
			test_const(&mut method, input, output);
		}
	}

	#[test]
	fn test_lowest1() {
		use super::{Lowest as TestingMethod, Method};
		let mut candles = RandomCandles::default();

		let mut ma = TestingMethod::new(1, candles.first().close);

		candles.take(100).for_each(|x| {
			assert_eq!(x.close, ma.next(x.close));
		});
	}

	#[test]
	fn test_lowest() {
		use super::{Lowest as TestingMethod, Method};
		let candles = RandomCandles::default();

		let src: Vec<ValueType> = candles.take(100).map(|x| x.close).collect();

		(2..20).for_each(|length| {
			let mut ma = TestingMethod::new(length, src[0]);
			let length = length as usize;

			src.iter().enumerate().for_each(|(i, &x)| {
				let value1 = ma.next(x);
				let value2 = (0..length).fold(src[i], |m, j| m.min(src[i.saturating_sub(j)]));
				assert_eq!(value2, value1);
			});
		});
	}

	#[test]
	fn test_highest_lowest_delta_const() {
		use super::*;
		use crate::core::{Candle, Method};
		use crate::methods::tests::test_const;

		for i in 1..30 {
			let input = (i as ValueType + 56.0) / 16.3251;
			let mut method = HighestLowestDelta::new(i, input);

			let output = method.next(input);
			test_const(&mut method, input, output);
		}
	}

	#[test]
	fn test_highes_lowest_delta1() {
		use super::{HighestLowestDelta as TestingMethod, Method};
		let mut candles = RandomCandles::default();

		let mut ma = TestingMethod::new(1, candles.first().close);

		candles.take(100).for_each(|x| {
			assert_eq!(0.0, ma.next(x.close));
		});
	}

	#[test]
	fn test_highes_lowest_delta() {
		use super::{HighestLowestDelta as TestingMethod, Method};
		let candles = RandomCandles::default();

		let src: Vec<ValueType> = candles.take(100).map(|x| x.close).collect();

		(2..20).for_each(|length| {
			let mut ma = TestingMethod::new(length, src[0]);
			let length = length as usize;

			src.iter().enumerate().for_each(|(i, &x)| {
				let value1 = ma.next(x);
				let min = (0..length).fold(src[i], |m, j| m.min(src[i.saturating_sub(j)]));
				let max = (0..length).fold(src[i], |m, j| m.max(src[i.saturating_sub(j)]));
				assert_eq!(max - min, value1);
			});
		});
	}
}
