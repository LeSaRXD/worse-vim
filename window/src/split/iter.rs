use lines::FILL_CHAR;

pub struct SplitIter<L>(SplitIterInner<L>);
impl<L> SplitIter<L> {
	pub fn single(lines: L) -> Self {
		Self(SplitIterInner::Single { lines })
	}
	pub fn horizontal(lines_list: Vec<L>) -> Self {
		Self(SplitIterInner::Horizontal { lines_list })
	}
	pub fn vertical(lines_list: Vec<L>, width: usize) -> Self {
		Self(SplitIterInner::Vertical { lines_list, width })
	}
}

enum SplitIterInner<L> {
	Single { lines: L },
	Horizontal { lines_list: Vec<L> },
	Vertical { lines_list: Vec<L>, width: usize },
}

impl<L> Iterator for SplitIter<L>
where
	L: Iterator<Item: Into<String>>,
{
	type Item = String;
	fn next(&mut self) -> Option<Self::Item> {
		match &mut self.0 {
			SplitIterInner::Single { lines } => lines.next().map(Into::into),
			SplitIterInner::Horizontal { lines_list } => {
				let mut total = Vec::new();
				for lines in lines_list {
					total.push(lines.next()?.into());
				}
				Some(total.join(FILL_CHAR))
			}
			SplitIterInner::Vertical { lines_list, width } => match lines_list.last_mut() {
				Some(lines) => lines.next().map(Into::into),
				None => {
					lines_list.pop();
					(!lines_list.is_empty()).then(|| FILL_CHAR.repeat(*width))
				}
			},
		}
	}
}
