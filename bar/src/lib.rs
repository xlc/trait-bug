pub use paste;
pub trait Key {}

pub trait AggregratedKeyValue {
    type AggregratedKey;
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
			$vis enum $name {
				$(
					$key_name($key_name, Option<$value_type>),
				)*
			}

			$vis enum [<$name Key>] {
				$(
					$key_name($key_name),
				)*
			}

			impl $crate::AggregratedKeyValue for $name {
				type AggregratedKey = [<$name Key>];
			}

			$(
				$vis struct $key_name $( (pub $key_para) )?;

				impl $crate::Key for $key_name {}
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
		$vis enum $name {
			$(
				$parameter_name(<$parameter_type as $crate::AggregratedKeyValue>::AggregratedKey),
			)*
		}

		$(
			impl From<<$parameter_type as $crate::AggregratedKeyValue>::AggregratedKey> for $name {
				fn from(key: <$parameter_type as $crate::AggregratedKeyValue>::AggregratedKey) -> Self {
					$name::$parameter_name(key)
				}
			}
		)*
	};
}
