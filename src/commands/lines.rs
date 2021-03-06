use crate::commands::WholeStreamCommand;
use crate::data::Primitive;
use crate::errors::ShellError;
use crate::prelude::*;
use log::trace;

pub struct Lines;

impl WholeStreamCommand for Lines {
    fn name(&self) -> &str {
        "lines"
    }

    fn signature(&self) -> Signature {
        Signature::build("lines")
    }

    fn usage(&self) -> &str {
        "Split single string into rows, one per line."
    }

    fn run(
        &self,
        args: CommandArgs,
        registry: &CommandRegistry,
    ) -> Result<OutputStream, ShellError> {
        lines(args, registry)
    }
}

// TODO: "Amount remaining" wrapper

fn lines(args: CommandArgs, registry: &CommandRegistry) -> Result<OutputStream, ShellError> {
    let args = args.evaluate_once(registry)?;
    let tag = args.name_tag();
    let name_span = tag.span;
    let input = args.input;

    let stream = input
        .values
        .map(move |v| match v.value {
            UntaggedValue::Primitive(Primitive::String(s)) => {
                let split_result: Vec<_> = s.lines().filter(|s| s.trim() != "").collect();

                trace!("split result = {:?}", split_result);

                let mut result = VecDeque::new();
                for s in split_result {
                    result.push_back(ReturnSuccess::value(
                        UntaggedValue::Primitive(Primitive::String(s.into())).into_untagged_value(),
                    ));
                }
                result
            }
            _ => {
                let mut result = VecDeque::new();
                let value_span = v.tag.span;

                result.push_back(Err(ShellError::labeled_error_with_secondary(
                    "Expected a string from pipeline",
                    "requires string input",
                    name_span,
                    "value originates from here",
                    value_span,
                )));
                result
            }
        })
        .flatten();

    Ok(stream.to_output_stream())
}
