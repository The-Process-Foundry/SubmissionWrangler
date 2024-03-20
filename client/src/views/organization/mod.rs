//! Display the Organizations

use yew::prelude::*;

use super::prelude::*;
use model::organization::Organization;

#[function_component(OrgGrid)]
pub fn org_grid() -> Html {
  let orgs = vec![
    Organization::sample("1 Org Test"),
    Organization::sample("2 Org Test"),
  ];

  let row_cls = classes!("text-left", "text-md");
  let rows: Vec<Html> = orgs
    .iter()
    .map(|org| {
      html! {
        <>
        <div class={row_cls.clone()}>{org.guid.to_string()}</div>
        <div class={row_cls.clone()}>{org.source_id}</div>
        <div class={row_cls.clone()}>{org.pretty_id.clone()}</div>
        <div class={row_cls.clone()}>{org.name.clone()}</div>
        </>
      }
    })
    .collect();

  let header_cls = classes!("text-left", "font-bold", "text-lg",);

  html! {
    <div id="org_grid" class={"p-4"}>
        <div class={"grid grid-rows-4 divide-x divide-gray"}>
            <div class={"col-span-4 text-center font-bold text-2xl"}>
                {"Organizations"}
            </div>
            <div class={header_cls.clone()}>{"GUID"}</div>
            <div class={header_cls.clone()}>{"SOURCE_ID"}</div>
            <div class={header_cls.clone()}>{"PRETTY_ID"}</div>
            <div class={header_cls.clone()}>{"NAME"}</div>
            {rows}

        </div>
  < /div>
  }
}
