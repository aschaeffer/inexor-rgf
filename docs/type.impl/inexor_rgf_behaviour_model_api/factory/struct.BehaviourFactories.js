(function() {var type_impls = {
"inexor_rgf_behaviour_model_impl":[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-BehaviourFactories%3CID,+T%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/inexor_rgf_behaviour_model_api/factory.rs.html#22\">source</a><a href=\"#impl-BehaviourFactories%3CID,+T%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;ID, T&gt; <a class=\"struct\" href=\"inexor_rgf_behaviour_model_api/factory/struct.BehaviourFactories.html\" title=\"struct inexor_rgf_behaviour_model_api::factory::BehaviourFactories\">BehaviourFactories</a>&lt;ID, T&gt;<div class=\"where\">where\n    ID: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,\n    T: <a class=\"trait\" href=\"inexor_rgf_reactive_model_api/instance/trait.ReactiveInstance.html\" title=\"trait inexor_rgf_reactive_model_api::instance::ReactiveInstance\">ReactiveInstance</a>&lt;ID&gt;,</div></h3></section></summary><div class=\"impl-items\"><section id=\"method.new\" class=\"method\"><a class=\"src rightside\" href=\"src/inexor_rgf_behaviour_model_api/factory.rs.html#23\">source</a><h4 class=\"code-header\">pub fn <a href=\"inexor_rgf_behaviour_model_api/factory/struct.BehaviourFactories.html#tymethod.new\" class=\"fn\">new</a>() -&gt; <a class=\"struct\" href=\"inexor_rgf_behaviour_model_api/factory/struct.BehaviourFactories.html\" title=\"struct inexor_rgf_behaviour_model_api::factory::BehaviourFactories\">BehaviourFactories</a>&lt;ID, T&gt;</h4></section><section id=\"method.factory\" class=\"method\"><a class=\"src rightside\" href=\"src/inexor_rgf_behaviour_model_api/factory.rs.html#27\">source</a><h4 class=\"code-header\">pub fn <a href=\"inexor_rgf_behaviour_model_api/factory/struct.BehaviourFactories.html#tymethod.factory\" class=\"fn\">factory</a>(\n    self,\n    factory: <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html\" title=\"struct alloc::sync::Arc\">Arc</a>&lt;dyn <a class=\"trait\" href=\"inexor_rgf_behaviour_model_api/factory/trait.BehaviourFactory.html\" title=\"trait inexor_rgf_behaviour_model_api::factory::BehaviourFactory\">BehaviourFactory</a>&lt;ID, T&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>&gt;\n) -&gt; <a class=\"struct\" href=\"inexor_rgf_behaviour_model_api/factory/struct.BehaviourFactories.html\" title=\"struct inexor_rgf_behaviour_model_api::factory::BehaviourFactories\">BehaviourFactories</a>&lt;ID, T&gt;</h4></section><section id=\"method.push\" class=\"method\"><a class=\"src rightside\" href=\"src/inexor_rgf_behaviour_model_api/factory.rs.html#32\">source</a><h4 class=\"code-header\">pub fn <a href=\"inexor_rgf_behaviour_model_api/factory/struct.BehaviourFactories.html#tymethod.push\" class=\"fn\">push</a>(&amp;self, factory: <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html\" title=\"struct alloc::sync::Arc\">Arc</a>&lt;dyn <a class=\"trait\" href=\"inexor_rgf_behaviour_model_api/factory/trait.BehaviourFactory.html\" title=\"trait inexor_rgf_behaviour_model_api::factory::BehaviourFactory\">BehaviourFactory</a>&lt;ID, T&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>&gt;)</h4></section></div></details>",0,"inexor_rgf_behaviour_model_impl::entity::factory::EntityBehaviourFactories"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Clone-for-BehaviourFactories%3CID,+T%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/inexor_rgf_behaviour_model_api/factory.rs.html#45\">source</a><a href=\"#impl-Clone-for-BehaviourFactories%3CID,+T%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;ID, T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"inexor_rgf_behaviour_model_api/factory/struct.BehaviourFactories.html\" title=\"struct inexor_rgf_behaviour_model_api::factory::BehaviourFactories\">BehaviourFactories</a>&lt;ID, T&gt;<div class=\"where\">where\n    ID: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,\n    T: <a class=\"trait\" href=\"inexor_rgf_reactive_model_api/instance/trait.ReactiveInstance.html\" title=\"trait inexor_rgf_reactive_model_api::instance::ReactiveInstance\">ReactiveInstance</a>&lt;ID&gt;,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/inexor_rgf_behaviour_model_api/factory.rs.html#46\">source</a><a href=\"#method.clone\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone\" class=\"fn\">clone</a>(&amp;self) -&gt; <a class=\"struct\" href=\"inexor_rgf_behaviour_model_api/factory/struct.BehaviourFactories.html\" title=\"struct inexor_rgf_behaviour_model_api::factory::BehaviourFactories\">BehaviourFactories</a>&lt;ID, T&gt;</h4></section></summary><div class='docblock'>Returns a copy of the value. <a href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone_from\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/nightly/src/core/clone.rs.html#169\">source</a></span><a href=\"#method.clone_from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from\" class=\"fn\">clone_from</a>(&amp;mut self, source: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;Self</a>)</h4></section></summary><div class='docblock'>Performs copy-assignment from <code>source</code>. <a href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from\">Read more</a></div></details></div></details>","Clone","inexor_rgf_behaviour_model_impl::entity::factory::EntityBehaviourFactories"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Deref-for-BehaviourFactories%3CID,+T%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/inexor_rgf_behaviour_model_api/factory.rs.html#37\">source</a><a href=\"#impl-Deref-for-BehaviourFactories%3CID,+T%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;ID, T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a> for <a class=\"struct\" href=\"inexor_rgf_behaviour_model_api/factory/struct.BehaviourFactories.html\" title=\"struct inexor_rgf_behaviour_model_api::factory::BehaviourFactories\">BehaviourFactories</a>&lt;ID, T&gt;<div class=\"where\">where\n    ID: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,\n    T: <a class=\"trait\" href=\"inexor_rgf_reactive_model_api/instance/trait.ReactiveInstance.html\" title=\"trait inexor_rgf_reactive_model_api::instance::ReactiveInstance\">ReactiveInstance</a>&lt;ID&gt;,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle\" open><summary><section id=\"associatedtype.Target\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.Target\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target\" class=\"associatedtype\">Target</a> = DashMap&lt;<a class=\"struct\" href=\"inexor_rgf_behaviour_model_api/types/behaviour_type_id/struct.BehaviourTypeId.html\" title=\"struct inexor_rgf_behaviour_model_api::types::behaviour_type_id::BehaviourTypeId\">BehaviourTypeId</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html\" title=\"struct alloc::sync::Arc\">Arc</a>&lt;dyn <a class=\"trait\" href=\"inexor_rgf_behaviour_model_api/factory/trait.BehaviourFactory.html\" title=\"trait inexor_rgf_behaviour_model_api::factory::BehaviourFactory\">BehaviourFactory</a>&lt;ID, T&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>&gt;&gt;</h4></section></summary><div class='docblock'>The resulting type after dereferencing.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.deref\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/inexor_rgf_behaviour_model_api/factory.rs.html#40\">source</a><a href=\"#method.deref\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#tymethod.deref\" class=\"fn\">deref</a>(&amp;self) -&gt; &amp;&lt;<a class=\"struct\" href=\"inexor_rgf_behaviour_model_api/factory/struct.BehaviourFactories.html\" title=\"struct inexor_rgf_behaviour_model_api::factory::BehaviourFactories\">BehaviourFactories</a>&lt;ID, T&gt; as <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a>&gt;::<a class=\"associatedtype\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target\" title=\"type core::ops::deref::Deref::Target\">Target</a></h4></section></summary><div class='docblock'>Dereferences the value.</div></details></div></details>","Deref","inexor_rgf_behaviour_model_impl::entity::factory::EntityBehaviourFactories"]]
};if (window.register_type_impls) {window.register_type_impls(type_impls);} else {window.pending_type_impls = type_impls;}})()