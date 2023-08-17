pub use paste;

pub trait RuntimeParameterStore {
	type AggregratedKeyValue: AggregratedKeyValue;

	fn get<KV, K>(key: K) -> Option<K::Value>
	where
		KV: AggregratedKeyValue,
		K: Key + Into<<KV as AggregratedKeyValue>::AggregratedKey>,
		<KV as AggregratedKeyValue>::AggregratedKey:
			Into<<<Self as RuntimeParameterStore>::AggregratedKeyValue as AggregratedKeyValue>::AggregratedKey>,
		<<Self as RuntimeParameterStore>::AggregratedKeyValue as AggregratedKeyValue>::AggregratedValue:
			TryInto<<KV as AggregratedKeyValue>::AggregratedValue>,
		<KV as AggregratedKeyValue>::AggregratedValue: TryInto<K::WrappedValue>;
}

pub trait Key {
	type Value;
	type WrappedValue: Into<Self::Value>;
}

pub trait AggregratedKeyValue {
	type AggregratedKey;
	type AggregratedValue;

	fn into_parts(self) -> (Self::AggregratedKey, Option<Self::AggregratedValue>);
}

#[macro_export]
macro_rules! define_parameters {
	(
		$vis:vis $name:ident = {
			$(
				$key_name:ident $( ($key_para: ty) )? : $value_type:ty = $index:expr
			),+ $(,)?
		}
	) => {
		$crate::paste::item! {
			#[derive(
				Clone,
				PartialEq,
				Eq,
			)]
			$vis enum $name {
				$(
					$key_name($key_name, Option<$value_type>),
				)*
			}

			#[derive(
				Clone,
				PartialEq,
				Eq,
			)]
			$vis enum [<$name Key>] {
				$(
					$key_name($key_name),
				)*
			}

			#[derive(
				Clone,
				PartialEq,
				Eq,
			)]
			$vis enum [<$name Value>] {
				$(
					$key_name($value_type),
				)*
			}

			impl $crate::AggregratedKeyValue for $name {
				type AggregratedKey = [<$name Key>];
				type AggregratedValue = [<$name Value>];

				fn into_parts(self) -> (Self::AggregratedKey, Option<Self::AggregratedValue>) {
					match self {
						$(
							$name::$key_name(key, value) => ([<$name Key>]::$key_name(key), value.map([<$name Value>]::$key_name)),
						)*
					}
				}
			}

			$(
				#[derive(
					Clone,
					PartialEq,
					Eq,
				)]
				$vis struct $key_name $( (pub $key_para) )?;

				impl $crate::Key for $key_name {
					type Value = $value_type;
					type WrappedValue = [<$key_name Value>];
				}

				impl From<$key_name> for [<$name Key>] {
					fn from(key: $key_name) -> Self {
						[<$name Key>]::$key_name(key)
					}
				}

				impl TryFrom<[<$name Key>]> for $key_name {
					type Error = ();

					fn try_from(key: [<$name Key>]) -> Result<Self, Self::Error> {
						match key {
							[<$name Key>]::$key_name(key) => Ok(key),
							_ => Err(()),
						}
					}
				}

				#[derive(
					Clone,
					PartialEq,
					Eq,
				)]
				$vis struct [<$key_name Value>](pub $value_type);

				impl From<[<$key_name Value>]> for [<$name Value>] {
					fn from(value: [<$key_name Value>]) -> Self {
						[<$name Value>]::$key_name(value.0)
					}
				}

				impl From<($key_name, $value_type)> for $name {
					fn from((key, value): ($key_name, $value_type)) -> Self {
						$name::$key_name(key, Some(value))
					}
				}

				impl From<$key_name> for $name {
					fn from(key: $key_name) -> Self {
						$name::$key_name(key, None)
					}
				}

				impl TryFrom<[<$name Value>]> for [<$key_name Value>] {
					type Error = ();

					fn try_from(value: [<$name Value>]) -> Result<Self, Self::Error> {
						match value {
							[<$name Value>]::$key_name(value) => Ok([<$key_name Value>](value)),
							_ => Err(()),
						}
					}
				}

				impl From<[<$key_name Value>]> for $value_type {
					fn from(value: [<$key_name Value>]) -> Self {
						value.0
					}
				}
			)*
		}
	};
}

#[macro_export]
macro_rules! define_aggregrated_parameters {
	(
		$vis:vis $name:ident = {
			$(
				$parameter_name:ident: $parameter_type:ty = $index:expr
			),+ $(,)?
		}
	) => {
		$crate::paste::item! {
			#[derive(
				Clone,
				PartialEq,
				Eq,
			)]
			$vis enum $name {
				$(
					$parameter_name($parameter_type),
				)*
			}

			#[derive(
				Clone,
				PartialEq,
				Eq,
			)]
			$vis enum [<$name Key>] {
				$(
					$parameter_name(<$parameter_type as $crate::AggregratedKeyValue>::AggregratedKey),
				)*
			}

			#[derive(
				Clone,
				PartialEq,
				Eq,
			)]
			$vis enum [<$name Value>] {
				$(
					$parameter_name(<$parameter_type as $crate::AggregratedKeyValue>::AggregratedValue),
				)*
			}

			impl $crate::AggregratedKeyValue for $name {
				type AggregratedKey = [<$name Key>];
				type AggregratedValue = [<$name Value>];

				fn into_parts(self) -> (Self::AggregratedKey, Option<Self::AggregratedValue>) {
					match self {
						$(
							$name::$parameter_name(parameter) => {
								let (key, value) = parameter.into_parts();
								([<$name Key>]::$parameter_name(key), value.map([<$name Value>]::$parameter_name))
							},
						)*
					}
				}
			}

			// $(
			// 	impl From<<$parameter_type as $crate::AggregratedKeyValue>::AggregratedKey> for [<$name Key>] {
			// 		fn from(key: <$parameter_type as $crate::AggregratedKeyValue>::AggregratedKey) -> Self {
			// 			[<$name Key>]::$parameter_name(key)
			// 		}
			// 	}

			// 	impl TryFrom<[<$name Value>]> for <$parameter_type as $crate::AggregratedKeyValue>::AggregratedValue {
			// 		type Error = ();

			// 		fn try_from(value: [<$name Value>]) -> Result<Self, Self::Error> {
			// 			match value {
			// 				[<$name Value>]::$parameter_name(value) => Ok(value),
			// 				_ => Err(()),
			// 			}
			// 		}
			// 	}
			// )*

				struct Test;

			$(
				impl Into<Test> for <$parameter_type as $crate::AggregratedKeyValue>::AggregratedValue {
					fn into(self) -> Test {
						unimplemented!()
					}
				}

				// impl TryFrom<[<$name Value>]> for <$parameter_type as $crate::AggregratedKeyValue>::AggregratedValue {
				// 	type Error = ();

				// 	fn try_from(value: [<$name Value>]) -> Result<Self, Self::Error> {
				// 		match value {
				// 			[<$name Value>]::$parameter_name(value) => Ok(value),
				// 			_ => Err(()),
				// 		}
				// 	}
				// }
			)*
		}
	};
}
