(function() {
    var type_impls = Object.fromEntries([["reactive_graph",[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-From%3COption%3COutputFormatArgs%3E%3E-for-OutputFormatWrapper%3CS,+T,+O%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/reactive_graph/client/output_format.rs.html#18-22\">source</a><a href=\"#impl-From%3COption%3COutputFormatArgs%3E%3E-for-OutputFormatWrapper%3CS,+T,+O%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;S: <a class=\"trait\" href=\"https://docs.rs/serde/1.0.213/serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a>, T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + Tabled + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;S&gt; + <a class=\"trait\" href=\"reactive_graph_table_model/container/trait.TableInlineFormatSetter.html\" title=\"trait reactive_graph_table_model::container::TableInlineFormatSetter\">TableInlineFormatSetter</a>, O: <a class=\"trait\" href=\"reactive_graph_table_model/container/trait.TableOptions.html\" title=\"trait reactive_graph_table_model::container::TableOptions\">TableOptions</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;<a class=\"enum\" href=\"reactive_graph/shared/output_format/enum.OutputFormatArgs.html\" title=\"enum reactive_graph::shared::output_format::OutputFormatArgs\">OutputFormatArgs</a>&gt;&gt; for <a class=\"struct\" href=\"reactive_graph/client/output_format/struct.OutputFormatWrapper.html\" title=\"struct reactive_graph::client::output_format::OutputFormatWrapper\">OutputFormatWrapper</a>&lt;S, T, O&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.from\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/reactive_graph/client/output_format.rs.html#19-21\">source</a><a href=\"#method.from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from\" class=\"fn\">from</a>(value: <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;<a class=\"enum\" href=\"reactive_graph/shared/output_format/enum.OutputFormatArgs.html\" title=\"enum reactive_graph::shared::output_format::OutputFormatArgs\">OutputFormatArgs</a>&gt;) -&gt; Self</h4></section></summary><div class='docblock'>Converts to this type from the input type.</div></details></div></details>","From<Option<OutputFormatArgs>>","reactive_graph::client::instances::entities::output_format::EntityInstancesOutputFormatWrapper","reactive_graph::client::instances::properties::output_format::PropertyInstancesOutputFormatWrapper","reactive_graph::client::instances::relations::output_format::RelationInstancesOutputFormatWrapper","reactive_graph::client::types::components::output_format::ComponentsOutputFormatWrapper","reactive_graph::client::types::components::output_format::ComponentTypeIdsOutputFormatWrapper","reactive_graph::client::types::entities::output_format::EntityTypesOutputFormatWrapper","reactive_graph::client::types::extension::output_format::ExtensionsOutputFormatWrapper","reactive_graph::client::types::property_type::output_format::PropertyTypesOutputFormatWrapper","reactive_graph::client::types::relations::output_format::RelationTypesOutputFormatWrapper"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-OutputFormatWrapper%3CS,+T,+O%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/reactive_graph/client/output_format.rs.html#24-31\">source</a><a href=\"#impl-OutputFormatWrapper%3CS,+T,+O%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;S: <a class=\"trait\" href=\"https://docs.rs/serde/1.0.213/serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> + 'static, T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + Tabled + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;S&gt; + <a class=\"trait\" href=\"reactive_graph_table_model/container/trait.TableInlineFormatSetter.html\" title=\"trait reactive_graph_table_model::container::TableInlineFormatSetter\">TableInlineFormatSetter</a> + 'static, O: <a class=\"trait\" href=\"reactive_graph_table_model/container/trait.TableOptions.html\" title=\"trait reactive_graph_table_model::container::TableOptions\">TableOptions</a> + 'static&gt; <a class=\"struct\" href=\"reactive_graph/client/output_format/struct.OutputFormatWrapper.html\" title=\"struct reactive_graph::client::output_format::OutputFormatWrapper\">OutputFormatWrapper</a>&lt;S, T, O&gt;</h3></section></summary><div class=\"impl-items\"><section id=\"method.single\" class=\"method\"><a class=\"src rightside\" href=\"src/reactive_graph/client/output_format.rs.html#25-27\">source</a><h4 class=\"code-header\">pub(crate) fn <a href=\"reactive_graph/client/output_format/struct.OutputFormatWrapper.html#tymethod.single\" class=\"fn\">single</a>(self, single_object: S) -&gt; <a class=\"type\" href=\"reactive_graph/client/result/type.CommandResult.html\" title=\"type reactive_graph::client::result::CommandResult\">CommandResult</a></h4></section><section id=\"method.collection\" class=\"method\"><a class=\"src rightside\" href=\"src/reactive_graph/client/output_format.rs.html#28-30\">source</a><h4 class=\"code-header\">pub(crate) fn <a href=\"reactive_graph/client/output_format/struct.OutputFormatWrapper.html#tymethod.collection\" class=\"fn\">collection</a>(self, collection: <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;S&gt;) -&gt; <a class=\"type\" href=\"reactive_graph/client/result/type.CommandResult.html\" title=\"type reactive_graph::client::result::CommandResult\">CommandResult</a></h4></section></div></details>",0,"reactive_graph::client::instances::entities::output_format::EntityInstancesOutputFormatWrapper","reactive_graph::client::instances::properties::output_format::PropertyInstancesOutputFormatWrapper","reactive_graph::client::instances::relations::output_format::RelationInstancesOutputFormatWrapper","reactive_graph::client::types::components::output_format::ComponentsOutputFormatWrapper","reactive_graph::client::types::components::output_format::ComponentTypeIdsOutputFormatWrapper","reactive_graph::client::types::entities::output_format::EntityTypesOutputFormatWrapper","reactive_graph::client::types::extension::output_format::ExtensionsOutputFormatWrapper","reactive_graph::client::types::property_type::output_format::PropertyTypesOutputFormatWrapper","reactive_graph::client::types::relations::output_format::RelationTypesOutputFormatWrapper"]]]]);
    if (window.register_type_impls) {
        window.register_type_impls(type_impls);
    } else {
        window.pending_type_impls = type_impls;
    }
})()
//{"start":55,"fragment_lengths":[7257]}