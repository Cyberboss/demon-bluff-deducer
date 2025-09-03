use demon_bluff_gameplay_engine::{Expression, testimony::Testimony, villager::ConfirmedVillager};

pub fn build_expression_for_villager_set(
	confirmeds: &Vec<ConfirmedVillager>,
) -> Option<Expression<Testimony>> {
	let mut expression = None;
	for confirmed in confirmeds {
		let testimony_expression = match confirmed.instance().testimony() {
			Some(testimony) => {
				let testimony = testimony.clone();
				Some(
					if !confirmed.instance().archetype().cannot_lie()
						&& (confirmed.corrupted() || confirmed.true_identity().lies())
					{
						Expression::Not(Box::new(testimony))
					} else {
						testimony
					},
				)
			}
			None => None,
		};

		expression = match expression {
			Some(existing_expression) => Some(match testimony_expression {
				Some(testimony_expression) => Expression::And(
					Box::new(existing_expression),
					Box::new(testimony_expression),
				),
				None => existing_expression,
			}),
			None => testimony_expression,
		}
	}

	expression
}
