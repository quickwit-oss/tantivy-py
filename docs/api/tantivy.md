---
title: tantivy
---

<div>
    <main class="pdoc">
            <section class="module-info">
                    <h1 class="modulename">
tantivy    </h1>

                
                        <input id="mod-tantivy-view-source" class="view-source-toggle-state" type="checkbox" aria-hidden="true" tabindex="-1">

                        <label class="view-source-button" for="mod-tantivy-view-source"><span>View Source</span></label>

                        <div class="pdoc-code codehilite"><pre><span></span><span id="L-1"><a href="#L-1"><span class="linenos">1</span></a><span class="kn">from</span> <span class="nn">.tantivy</span> <span class="kn">import</span> <span class="o">*</span>
</span></pre></div>


            </section>
                <section id="Order">
                    <div class="attr class">
            
    <span class="def">class</span>
    <span class="name">Order</span>:

        
    </div>
    <a class="headerlink" href="#Order"></a>
    
            <div class="docstring"><p>Enum representing the direction in which something should be sorted.</p>
</div>


                            <div id="Order.Asc" class="classattr">
                                <div class="attr variable">
            <span class="name">Asc</span>        =
<span class="default_value"><a href="#Order.Asc">Order.Asc</a></span>

        
    </div>
    <a class="headerlink" href="#Order.Asc"></a>
    
    

                            </div>
                            <div id="Order.Desc" class="classattr">
                                <div class="attr variable">
            <span class="name">Desc</span>        =
<span class="default_value"><a href="#Order.Desc">Order.Desc</a></span>

        
    </div>
    <a class="headerlink" href="#Order.Desc"></a>
    
    

                            </div>
                </section>
                <section id="Schema">
                    <div class="attr class">
            
    <span class="def">class</span>
    <span class="name">Schema</span>:

        
    </div>
    <a class="headerlink" href="#Schema"></a>
    
            <div class="docstring"><p>Tantivy schema.</p>

<p>The schema is very strict. To build the schema the <code>SchemaBuilder</code> class is
provided.</p>
</div>


                </section>
                <section id="SearchResult">
                    <div class="attr class">
            
    <span class="def">class</span>
    <span class="name">SearchResult</span>:

        
    </div>
    <a class="headerlink" href="#SearchResult"></a>
    
            <div class="docstring"><p>Object holding a results successful search.</p>
</div>


                            <div id="SearchResult.count" class="classattr">
                                <div class="attr variable">
            <span class="name">count</span>

        
    </div>
    <a class="headerlink" href="#SearchResult.count"></a>
    
            <div class="docstring"><p>How many documents matched the query. Only available if <code><a href="#SearchResult.count">count</a></code> was set
to true during the search.</p>
</div>


                            </div>
                            <div id="SearchResult.hits" class="classattr">
                                <div class="attr variable">
            <span class="name">hits</span>

        
    </div>
    <a class="headerlink" href="#SearchResult.hits"></a>
    
            <div class="docstring"><p>The list of tuples that contains the scores and DocAddress of the
search results.</p>
</div>


                            </div>
                </section>
                <section id="Document">
                    <div class="attr class">
            
    <span class="def">class</span>
    <span class="name">Document</span>:

        
    </div>
    <a class="headerlink" href="#Document"></a>
    
            <div class="docstring"><p>Tantivy's Document is the object that can be indexed and then searched for.</p>

<p>Documents are fundamentally a collection of unordered tuples
(field_name, value). In this list, one field may appear more than once.</p>

<p>Example:</p>

<pre><code>&gt;&gt;&gt; doc = <a href="#Document">Document()</a>
&gt;&gt;&gt; doc.add_text("title", "The Old Man and the Sea")
&gt;&gt;&gt; doc.add_text("body", ("He was an old man who fished alone in a "
...                       "skiff in the Gulf Stream and he had gone "
...                       "eighty-four days now without taking a fish."))
&gt;&gt;&gt; doc
Document(body=[He was an ],title=[The Old Ma])
</code></pre>

<p>For simplicity, it is also possible to build a <code><a href="#Document">Document</a></code> by passing the field
values directly as constructor arguments.</p>

<p>Example:</p>

<pre><code>&gt;&gt;&gt; doc = <a href="#Document">Document</a>(title=["The Old Man and the Sea"], body=["..."])
</code></pre>

<p>As syntactic sugar, tantivy also allows the user to pass a single values
if there is only one. In other words, the following is also legal.</p>

<p>Example:</p>

<pre><code>&gt;&gt;&gt; doc = <a href="#Document">Document</a>(title="The Old Man and the Sea", body="...")
</code></pre>

<p>For numeric fields, the [<code><a href="#Document">Document</a></code>] constructor does not have any
information about the type and will try to guess the type.
Therefore, it is recommended to use the [<code>Document::from_dict()</code>],
[<code>Document::extract()</code>], or <code>Document::add_*()</code> functions to provide
explicit type information.</p>

<p>Example:</p>

<pre><code>&gt;&gt;&gt; schema = (
...     SchemaBuilder()
...         .add_unsigned_field("unsigned")
...         .add_integer_field("signed")
...         .add_float_field("float")
...         .build()
... )
&gt;&gt;&gt; doc = <a href="#Document.from_dict">Document.from_dict</a>(
...     {"unsigned": 1000, "signed": -5, "float": 0.4},
...     schema,
... )
</code></pre>
</div>


                            <div id="Document.extend" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">extend</span><span class="signature pdoc-code condensed">(<span class="param"><span class="bp">self</span>, </span><span class="param"><span class="o">/</span>, </span><span class="param"><span class="n">py_dict</span>, </span><span class="param"><span class="n">schema</span><span class="o">=</span><span class="kc">None</span></span><span class="return-annotation">):</span></span>

        
    </div>
    <a class="headerlink" href="#Document.extend"></a>
    
    

                            </div>
                            <div id="Document.from_dict" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">from_dict</span><span class="signature pdoc-code condensed">(<span class="param"><span class="n">py_dict</span>, </span><span class="param"><span class="n">schema</span><span class="o">=</span><span class="kc">None</span></span><span class="return-annotation">):</span></span>

        
    </div>
    <a class="headerlink" href="#Document.from_dict"></a>
    
    

                            </div>
                            <div id="Document.to_dict" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">to_dict</span><span class="signature pdoc-code condensed">(<span class="param"><span class="bp">self</span>, </span><span class="param"><span class="o">/</span></span><span class="return-annotation">):</span></span>

        
    </div>
    <a class="headerlink" href="#Document.to_dict"></a>
    
            <div class="docstring"><p>Returns a dictionary with the different
field values.</p>

<p>In tantivy, <code><a href="#Document">Document</a></code> can be hold multiple
values for a single field.</p>

<p>For this reason, the dictionary, will associate
a list of value for every field.</p>
</div>


                            </div>
                            <div id="Document.add_text" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">add_text</span><span class="signature pdoc-code condensed">(<span class="param"><span class="bp">self</span>, </span><span class="param"><span class="o">/</span>, </span><span class="param"><span class="n">field_name</span>, </span><span class="param"><span class="n">text</span></span><span class="return-annotation">):</span></span>

        
    </div>
    <a class="headerlink" href="#Document.add_text"></a>
    
            <div class="docstring"><p>Add a text value to the document.</p>

<p>Args:
    field_name (str): The field name for which we are adding the text.
    text (str): The text that will be added to the document.</p>
</div>


                            </div>
                            <div id="Document.add_unsigned" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">add_unsigned</span><span class="signature pdoc-code condensed">(<span class="param"><span class="bp">self</span>, </span><span class="param"><span class="o">/</span>, </span><span class="param"><span class="n">field_name</span>, </span><span class="param"><span class="n">value</span></span><span class="return-annotation">):</span></span>

        
    </div>
    <a class="headerlink" href="#Document.add_unsigned"></a>
    
            <div class="docstring"><p>Add an unsigned integer value to the document.</p>

<p>Args:
    field_name (str): The field name for which we are adding the unsigned integer.
    value (int): The integer that will be added to the document.</p>
</div>


                            </div>
                            <div id="Document.add_integer" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">add_integer</span><span class="signature pdoc-code condensed">(<span class="param"><span class="bp">self</span>, </span><span class="param"><span class="o">/</span>, </span><span class="param"><span class="n">field_name</span>, </span><span class="param"><span class="n">value</span></span><span class="return-annotation">):</span></span>

        
    </div>
    <a class="headerlink" href="#Document.add_integer"></a>
    
            <div class="docstring"><p>Add a signed integer value to the document.</p>

<p>Args:
    field_name (str): The field name for which we are adding the integer.
    value (int): The integer that will be added to the document.</p>
</div>


                            </div>
                            <div id="Document.add_float" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">add_float</span><span class="signature pdoc-code condensed">(<span class="param"><span class="bp">self</span>, </span><span class="param"><span class="o">/</span>, </span><span class="param"><span class="n">field_name</span>, </span><span class="param"><span class="n">value</span></span><span class="return-annotation">):</span></span>

        
    </div>
    <a class="headerlink" href="#Document.add_float"></a>
    
            <div class="docstring"><p>Add a float value to the document.</p>

<p>Args:
    field_name (str): The field name for which we are adding the value.
    value (f64): The float that will be added to the document.</p>
</div>


                            </div>
                            <div id="Document.add_boolean" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">add_boolean</span><span class="signature pdoc-code condensed">(<span class="param"><span class="bp">self</span>, </span><span class="param"><span class="o">/</span>, </span><span class="param"><span class="n">field_name</span>, </span><span class="param"><span class="n">value</span></span><span class="return-annotation">):</span></span>

        
    </div>
    <a class="headerlink" href="#Document.add_boolean"></a>
    
            <div class="docstring"><p>Add a boolean value to the document.</p>

<p>Args:
    field_name (str): The field name for which we are adding the value.
    value (bool): The boolean that will be added to the document.</p>
</div>


                            </div>
                            <div id="Document.add_date" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">add_date</span><span class="signature pdoc-code condensed">(<span class="param"><span class="bp">self</span>, </span><span class="param"><span class="o">/</span>, </span><span class="param"><span class="n">field_name</span>, </span><span class="param"><span class="n">value</span></span><span class="return-annotation">):</span></span>

        
    </div>
    <a class="headerlink" href="#Document.add_date"></a>
    
            <div class="docstring"><p>Add a date value to the document.</p>

<p>Args:
    field_name (str): The field name for which we are adding the date.
    value (datetime): The date that will be added to the document.</p>
</div>


                            </div>
                            <div id="Document.add_facet" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">add_facet</span><span class="signature pdoc-code condensed">(<span class="param"><span class="bp">self</span>, </span><span class="param"><span class="o">/</span>, </span><span class="param"><span class="n">field_name</span>, </span><span class="param"><span class="n">facet</span></span><span class="return-annotation">):</span></span>

        
    </div>
    <a class="headerlink" href="#Document.add_facet"></a>
    
            <div class="docstring"><p>Add a facet value to the document.
Args:
    field_name (str): The field name for which we are adding the facet.
    value (Facet): The Facet that will be added to the document.</p>
</div>


                            </div>
                            <div id="Document.add_bytes" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">add_bytes</span><span class="signature pdoc-code condensed">(<span class="param"><span class="bp">self</span>, </span><span class="param"><span class="o">/</span>, </span><span class="param"><span class="n">field_name</span>, </span><span class="param"><span class="nb">bytes</span></span><span class="return-annotation">):</span></span>

        
    </div>
    <a class="headerlink" href="#Document.add_bytes"></a>
    
            <div class="docstring"><p>Add a bytes value to the document.</p>

<p>Args:
    field_name (str): The field for which we are adding the bytes.
    value (bytes): The bytes that will be added to the document.</p>
</div>


                            </div>
                            <div id="Document.add_json" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">add_json</span><span class="signature pdoc-code condensed">(<span class="param"><span class="bp">self</span>, </span><span class="param"><span class="o">/</span>, </span><span class="param"><span class="n">field_name</span>, </span><span class="param"><span class="n">value</span></span><span class="return-annotation">):</span></span>

        
    </div>
    <a class="headerlink" href="#Document.add_json"></a>
    
            <div class="docstring"><p>Add a JSON value to the document.</p>

<p>Args:
    field_name (str): The field for which we are adding the bytes.
    value (str | Dict[str, Any]): The JSON object that will be added
        to the document.</p>

<p>Raises a ValueError if the JSON is invalid.</p>
</div>


                            </div>
                            <div id="Document.get_first" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">get_first</span><span class="signature pdoc-code condensed">(<span class="param"><span class="bp">self</span>, </span><span class="param"><span class="o">/</span>, </span><span class="param"><span class="n">fieldname</span></span><span class="return-annotation">):</span></span>

        
    </div>
    <a class="headerlink" href="#Document.get_first"></a>
    
            <div class="docstring"><p>Get the first value associated with the given field.</p>

<p>Args:
    field (Field): The field for which we would like to get the value.</p>

<p>Returns the value if one is found, otherwise None.
The type of the value depends on the field.</p>
</div>


                            </div>
                            <div id="Document.get_all" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">get_all</span><span class="signature pdoc-code condensed">(<span class="param"><span class="bp">self</span>, </span><span class="param"><span class="o">/</span>, </span><span class="param"><span class="n">field_name</span></span><span class="return-annotation">):</span></span>

        
    </div>
    <a class="headerlink" href="#Document.get_all"></a>
    
            <div class="docstring"><p>Get the all values associated with the given field.</p>

<p>Args:
    field (Field): The field for which we would like to get the values.</p>

<p>Returns a list of values.
The type of the value depends on the field.</p>
</div>


                            </div>
                            <div id="Document.num_fields" class="classattr">
                                <div class="attr variable">
            <span class="name">num_fields</span>

        
    </div>
    <a class="headerlink" href="#Document.num_fields"></a>
    
            <div class="docstring"><p>Returns the number of added fields that have been added to the document</p>
</div>


                            </div>
                            <div id="Document.is_empty" class="classattr">
                                <div class="attr variable">
            <span class="name">is_empty</span>

        
    </div>
    <a class="headerlink" href="#Document.is_empty"></a>
    
            <div class="docstring"><p>True if the document is empty, False otherwise.</p>
</div>


                            </div>
                </section>
                <section id="DocAddress">
                    <div class="attr class">
            
    <span class="def">class</span>
    <span class="name">DocAddress</span>:

        
    </div>
    <a class="headerlink" href="#DocAddress"></a>
    
            <div class="docstring"><p>DocAddress contains all the necessary information to identify a document
given a Searcher object.</p>

<p>It consists in an id identifying its segment, and its segment-local DocId.
The id used for the segment is actually an ordinal in the list of segment
hold by a Searcher.</p>
</div>


                            <div id="DocAddress.segment_ord" class="classattr">
                                <div class="attr variable">
            <span class="name">segment_ord</span>

        
    </div>
    <a class="headerlink" href="#DocAddress.segment_ord"></a>
    
            <div class="docstring"><p>The segment ordinal is an id identifying the segment hosting the
document. It is only meaningful, in the context of a searcher.</p>
</div>


                            </div>
                            <div id="DocAddress.doc" class="classattr">
                                <div class="attr variable">
            <span class="name">doc</span>

        
    </div>
    <a class="headerlink" href="#DocAddress.doc"></a>
    
            <div class="docstring"><p>The segment local DocId</p>
</div>


                            </div>
                </section>
                <section id="Facet">
                    <div class="attr class">
            
    <span class="def">class</span>
    <span class="name">Facet</span>:

        
    </div>
    <a class="headerlink" href="#Facet"></a>
    
            <div class="docstring"><p>A Facet represent a point in a given hierarchy.</p>

<p>They are typically represented similarly to a filepath. For instance, an
e-commerce website could have a Facet for /electronics/tv_and_video/led_tv.</p>

<p>A document can be associated to any number of facets. The hierarchy
implicitely imply that a document belonging to a facet also belongs to the
ancestor of its facet. In the example above, /electronics/tv_and_video/
and /electronics.</p>
</div>


                            <div id="Facet.from_encoded" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">from_encoded</span><span class="signature pdoc-code condensed">(<span class="param"><span class="n">encoded_bytes</span></span><span class="return-annotation">):</span></span>

        
    </div>
    <a class="headerlink" href="#Facet.from_encoded"></a>
    
            <div class="docstring"><p>Creates a <code><a href="#Facet">Facet</a></code> from its binary representation.</p>
</div>


                            </div>
                            <div id="Facet.root" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">root</span><span class="signature pdoc-code condensed">(<span class="param"><span class="bp">cls</span>, </span><span class="param"><span class="o">/</span></span><span class="return-annotation">):</span></span>

        
    </div>
    <a class="headerlink" href="#Facet.root"></a>
    
            <div class="docstring"><p>Create a new instance of the "root facet" Equivalent to /.</p>
</div>


                            </div>
                            <div id="Facet.is_prefix_of" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">is_prefix_of</span><span class="signature pdoc-code condensed">(<span class="param"><span class="bp">self</span>, </span><span class="param"><span class="o">/</span>, </span><span class="param"><span class="n">other</span></span><span class="return-annotation">):</span></span>

        
    </div>
    <a class="headerlink" href="#Facet.is_prefix_of"></a>
    
            <div class="docstring"><p>Returns true if another Facet is a subfacet of this facet.
Args:
    other (Facet): The Facet that we should check if this facet is a
        subset of.</p>
</div>


                            </div>
                            <div id="Facet.from_string" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">from_string</span><span class="signature pdoc-code condensed">(<span class="param"><span class="bp">cls</span>, </span><span class="param"><span class="o">/</span>, </span><span class="param"><span class="n">facet_string</span></span><span class="return-annotation">):</span></span>

        
    </div>
    <a class="headerlink" href="#Facet.from_string"></a>
    
            <div class="docstring"><p>Create a Facet object from a string.
Args:
    facet_string (str): The string that contains a facet.</p>

<p>Returns the created Facet.</p>
</div>


                            </div>
                            <div id="Facet.to_path" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">to_path</span><span class="signature pdoc-code condensed">(<span class="param"><span class="bp">self</span>, </span><span class="param"><span class="o">/</span></span><span class="return-annotation">):</span></span>

        
    </div>
    <a class="headerlink" href="#Facet.to_path"></a>
    
            <div class="docstring"><p>Returns the list of <code>segments</code> that forms a facet path.</p>

<p>For instance <code>//europe/france</code> becomes <code>["europe", "france"]</code>.</p>
</div>


                            </div>
                            <div id="Facet.to_path_str" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">to_path_str</span><span class="signature pdoc-code condensed">(<span class="param"><span class="bp">self</span>, </span><span class="param"><span class="o">/</span></span><span class="return-annotation">):</span></span>

        
    </div>
    <a class="headerlink" href="#Facet.to_path_str"></a>
    
            <div class="docstring"><p>Returns the facet string representation.</p>
</div>


                            </div>
                            <div id="Facet.is_root" class="classattr">
                                <div class="attr variable">
            <span class="name">is_root</span>

        
    </div>
    <a class="headerlink" href="#Facet.is_root"></a>
    
            <div class="docstring"><p>Returns true if the facet is the root facet /.</p>
</div>


                            </div>
                </section>
    </main>
<script>
    function escapeHTML(html) {
        return document.createElement('div').appendChild(document.createTextNode(html)).parentNode.innerHTML;
    }

    const originalContent = document.querySelector("main.pdoc");
    let currentContent = originalContent;

    function setContent(innerHTML) {
        let elem;
        if (innerHTML) {
            elem = document.createElement("main");
            elem.classList.add("pdoc");
            elem.innerHTML = innerHTML;
        } else {
            elem = originalContent;
        }
        if (currentContent !== elem) {
            currentContent.replaceWith(elem);
            currentContent = elem;
        }
    }

    function getSearchTerm() {
        return (new URL(window.location)).searchParams.get("search");
    }

    const searchBox = document.querySelector(".pdoc input[type=search]");
    searchBox.addEventListener("input", function () {
        let url = new URL(window.location);
        if (searchBox.value.trim()) {
            url.hash = "";
            url.searchParams.set("search", searchBox.value);
        } else {
            url.searchParams.delete("search");
        }
        history.replaceState("", "", url.toString());
        onInput();
    });
    window.addEventListener("popstate", onInput);


    let search, searchErr;

    async function initialize() {
        try {
            search = await new Promise((resolve, reject) => {
                const script = document.createElement("script");
                script.type = "text/javascript";
                script.async = true;
                script.onload = () => resolve(window.pdocSearch);
                script.onerror = (e) => reject(e);
                script.src = "search.js";
                document.getElementsByTagName("head")[0].appendChild(script);
            });
        } catch (e) {
            console.error("Cannot fetch pdoc search index");
            searchErr = "Cannot fetch search index.";
        }
        onInput();

        document.querySelector("nav.pdoc").addEventListener("click", e => {
            if (e.target.hash) {
                searchBox.value = "";
                searchBox.dispatchEvent(new Event("input"));
            }
        });
    }

    function onInput() {
        setContent((() => {
            const term = getSearchTerm();
            if (!term) {
                return null
            }
            if (searchErr) {
                return `<h3>Error: ${searchErr}</h3>`
            }
            if (!search) {
                return "<h3>Searching...</h3>"
            }

            window.scrollTo({top: 0, left: 0, behavior: 'auto'});

            const results = search(term);

            let html;
            if (results.length === 0) {
                html = `No search results for '${escapeHTML(term)}'.`
            } else {
                html = `<h4>${results.length} search result${results.length > 1 ? "s" : ""} for '${escapeHTML(term)}'.</h4>`;
            }
            for (let result of results.slice(0, 10)) {
                let doc = result.doc;
                let url = `${doc.modulename.replaceAll(".", "/")}.html`;
                if (doc.qualname) {
                    url += `#${doc.qualname}`;
                }

                let heading;
                switch (result.doc.kind) {
                    case "function":
                        if (doc.fullname.endsWith(".__init__")) {
                            heading = `<span class="name">${doc.fullname.replace(/\.__init__$/, "")}</span>${doc.signature}`;
                        } else {
                            heading = `<span class="def">${doc.funcdef}</span> <span class="name">${doc.fullname}</span>${doc.signature}`;
                        }
                        break;
                    case "class":
                        heading = `<span class="def">class</span> <span class="name">${doc.fullname}</span>`;
                        if (doc.bases)
                            heading += `<wbr>(<span class="base">${doc.bases}</span>)`;
                        heading += `:`;
                        break;
                    case "variable":
                        heading = `<span class="name">${doc.fullname}</span>`;
                        if (doc.annotation)
                            heading += `<span class="annotation">${doc.annotation}</span>`;
                        if (doc.default_value)
                            heading += `<span class="default_value"> = ${doc.default_value}</span>`;
                        break;
                    default:
                        heading = `<span class="name">${doc.fullname}</span>`;
                        break;
                }
                html += `
                        <section class="search-result">
                        <a href="${url}" class="attr ${doc.kind}">${heading}</a>
                        <div class="docstring">${doc.doc}</div>
                        </section>
                    `;

            }
            return html;
        })());
    }

    if (getSearchTerm()) {
        initialize();
        searchBox.value = getSearchTerm();
        onInput();
    } else {
        searchBox.addEventListener("focus", initialize, {once: true});
    }

    searchBox.addEventListener("keydown", e => {
        if (["ArrowDown", "ArrowUp", "Enter"].includes(e.key)) {
            let focused = currentContent.querySelector(".search-result.focused");
            if (!focused) {
                currentContent.querySelector(".search-result").classList.add("focused");
            } else if (
                e.key === "ArrowDown"
                && focused.nextElementSibling
                && focused.nextElementSibling.classList.contains("search-result")
            ) {
                focused.classList.remove("focused");
                focused.nextElementSibling.classList.add("focused");
                focused.nextElementSibling.scrollIntoView({
                    behavior: "smooth",
                    block: "nearest",
                    inline: "nearest"
                });
            } else if (
                e.key === "ArrowUp"
                && focused.previousElementSibling
                && focused.previousElementSibling.classList.contains("search-result")
            ) {
                focused.classList.remove("focused");
                focused.previousElementSibling.classList.add("focused");
                focused.previousElementSibling.scrollIntoView({
                    behavior: "smooth",
                    block: "nearest",
                    inline: "nearest"
                });
            } else if (
                e.key === "Enter"
            ) {
                focused.querySelector("a").click();
            }
        }
    });
</script>

<style>pre{line-height:125%;}span.linenos{color:inherit; background-color:transparent; padding-left:5px; padding-right:20px;}.pdoc-code .hll{background-color:#ffffcc}.pdoc-code{background:#f8f8f8;}.pdoc-code .c{color:#3D7B7B; font-style:italic}.pdoc-code .err{border:1px solid #FF0000}.pdoc-code .k{color:#008000; font-weight:bold}.pdoc-code .o{color:#666666}.pdoc-code .ch{color:#3D7B7B; font-style:italic}.pdoc-code .cm{color:#3D7B7B; font-style:italic}.pdoc-code .cp{color:#9C6500}.pdoc-code .cpf{color:#3D7B7B; font-style:italic}.pdoc-code .c1{color:#3D7B7B; font-style:italic}.pdoc-code .cs{color:#3D7B7B; font-style:italic}.pdoc-code .gd{color:#A00000}.pdoc-code .ge{font-style:italic}.pdoc-code .gr{color:#E40000}.pdoc-code .gh{color:#000080; font-weight:bold}.pdoc-code .gi{color:#008400}.pdoc-code .go{color:#717171}.pdoc-code .gp{color:#000080; font-weight:bold}.pdoc-code .gs{font-weight:bold}.pdoc-code .gu{color:#800080; font-weight:bold}.pdoc-code .gt{color:#0044DD}.pdoc-code .kc{color:#008000; font-weight:bold}.pdoc-code .kd{color:#008000; font-weight:bold}.pdoc-code .kn{color:#008000; font-weight:bold}.pdoc-code .kp{color:#008000}.pdoc-code .kr{color:#008000; font-weight:bold}.pdoc-code .kt{color:#B00040}.pdoc-code .m{color:#666666}.pdoc-code .s{color:#BA2121}.pdoc-code .na{color:#687822}.pdoc-code .nb{color:#008000}.pdoc-code .nc{color:#0000FF; font-weight:bold}.pdoc-code .no{color:#880000}.pdoc-code .nd{color:#AA22FF}.pdoc-code .ni{color:#717171; font-weight:bold}.pdoc-code .ne{color:#CB3F38; font-weight:bold}.pdoc-code .nf{color:#0000FF}.pdoc-code .nl{color:#767600}.pdoc-code .nn{color:#0000FF; font-weight:bold}.pdoc-code .nt{color:#008000; font-weight:bold}.pdoc-code .nv{color:#19177C}.pdoc-code .ow{color:#AA22FF; font-weight:bold}.pdoc-code .w{color:#bbbbbb}.pdoc-code .mb{color:#666666}.pdoc-code .mf{color:#666666}.pdoc-code .mh{color:#666666}.pdoc-code .mi{color:#666666}.pdoc-code .mo{color:#666666}.pdoc-code .sa{color:#BA2121}.pdoc-code .sb{color:#BA2121}.pdoc-code .sc{color:#BA2121}.pdoc-code .dl{color:#BA2121}.pdoc-code .sd{color:#BA2121; font-style:italic}.pdoc-code .s2{color:#BA2121}.pdoc-code .se{color:#AA5D1F; font-weight:bold}.pdoc-code .sh{color:#BA2121}.pdoc-code .si{color:#A45A77; font-weight:bold}.pdoc-code .sx{color:#008000}.pdoc-code .sr{color:#A45A77}.pdoc-code .s1{color:#BA2121}.pdoc-code .ss{color:#19177C}.pdoc-code .bp{color:#008000}.pdoc-code .fm{color:#0000FF}.pdoc-code .vc{color:#19177C}.pdoc-code .vg{color:#19177C}.pdoc-code .vi{color:#19177C}.pdoc-code .vm{color:#19177C}.pdoc-code .il{color:#666666}</style>
<style>:root{--pdoc-background:#fff;}.pdoc{--text:#212529;--muted:#6c757d;--link:#3660a5;--link-hover:#1659c5;--code:#f8f8f8;--active:#fff598;--accent:#eee;--accent2:#c1c1c1;--nav-hover:rgba(255, 255, 255, 0.5);--name:#0066BB;--def:#008800;--annotation:#007020;}</style>
<style>.pdoc{color:var(--text);box-sizing:border-box;line-height:1.5;background:none;}.pdoc .pdoc-button{cursor:pointer;display:inline-block;border:solid black 1px;border-radius:2px;font-size:.75rem;padding:calc(0.5em - 1px) 1em;transition:100ms all;}.pdoc .pdoc-alert{padding:1rem 1rem 1rem calc(1.5rem + 24px);border:1px solid transparent;border-radius:.25rem;background-repeat:no-repeat;background-position:1rem center;margin-bottom:1rem;}.pdoc .pdoc-alert > *:last-child{margin-bottom:0;}.pdoc .pdoc-alert-note {color:#084298;background-color:#cfe2ff;border-color:#b6d4fe;background-image:url("data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A//www.w3.org/2000/svg%22%20width%3D%2224%22%20height%3D%2224%22%20fill%3D%22%23084298%22%20viewBox%3D%220%200%2016%2016%22%3E%3Cpath%20d%3D%22M8%2016A8%208%200%201%200%208%200a8%208%200%200%200%200%2016zm.93-9.412-1%204.705c-.07.34.029.533.304.533.194%200%20.487-.07.686-.246l-.088.416c-.287.346-.92.598-1.465.598-.703%200-1.002-.422-.808-1.319l.738-3.468c.064-.293.006-.399-.287-.47l-.451-.081.082-.381%202.29-.287zM8%205.5a1%201%200%201%201%200-2%201%201%200%200%201%200%202z%22/%3E%3C/svg%3E");}.pdoc .pdoc-alert-warning{color:#664d03;background-color:#fff3cd;border-color:#ffecb5;background-image:url("data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A//www.w3.org/2000/svg%22%20width%3D%2224%22%20height%3D%2224%22%20fill%3D%22%23664d03%22%20viewBox%3D%220%200%2016%2016%22%3E%3Cpath%20d%3D%22M8.982%201.566a1.13%201.13%200%200%200-1.96%200L.165%2013.233c-.457.778.091%201.767.98%201.767h13.713c.889%200%201.438-.99.98-1.767L8.982%201.566zM8%205c.535%200%20.954.462.9.995l-.35%203.507a.552.552%200%200%201-1.1%200L7.1%205.995A.905.905%200%200%201%208%205zm.002%206a1%201%200%201%201%200%202%201%201%200%200%201%200-2z%22/%3E%3C/svg%3E");}.pdoc .pdoc-alert-danger{color:#842029;background-color:#f8d7da;border-color:#f5c2c7;background-image:url("data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A//www.w3.org/2000/svg%22%20width%3D%2224%22%20height%3D%2224%22%20fill%3D%22%23842029%22%20viewBox%3D%220%200%2016%2016%22%3E%3Cpath%20d%3D%22M5.52.359A.5.5%200%200%201%206%200h4a.5.5%200%200%201%20.474.658L8.694%206H12.5a.5.5%200%200%201%20.395.807l-7%209a.5.5%200%200%201-.873-.454L6.823%209.5H3.5a.5.5%200%200%201-.48-.641l2.5-8.5z%22/%3E%3C/svg%3E");}.pdoc .visually-hidden{position:absolute !important;width:1px !important;height:1px !important;padding:0 !important;margin:-1px !important;overflow:hidden !important;clip:rect(0, 0, 0, 0) !important;white-space:nowrap !important;border:0 !important;}.pdoc h1, .pdoc h2, .pdoc h3{font-weight:300;margin:.3em 0;padding:.2em 0;}.pdoc > section:not(.module-info) h1{font-size:1.5rem;font-weight:500;}.pdoc > section:not(.module-info) h2{font-size:1.4rem;font-weight:500;}.pdoc > section:not(.module-info) h3{font-size:1.3rem;font-weight:500;}.pdoc > section:not(.module-info) h4{font-size:1.2rem;}.pdoc > section:not(.module-info) h5{font-size:1.1rem;}.pdoc a{text-decoration:none;color:var(--link);}.pdoc a:hover{color:var(--link-hover);}.pdoc blockquote{margin-left:2rem;}.pdoc pre{border-top:1px solid var(--accent2);border-bottom:1px solid var(--accent2);margin-top:0;margin-bottom:1em;padding:.5rem 0 .5rem .5rem;overflow-x:auto;background-color:var(--code);}.pdoc code{color:var(--text);padding:.2em .4em;margin:0;font-size:85%;background-color:var(--accent);border-radius:6px;}.pdoc a > code{color:inherit;}.pdoc pre > code{display:inline-block;font-size:inherit;background:none;border:none;padding:0;}.pdoc > section:not(.module-info){margin-bottom:1.5rem;}.pdoc .modulename{margin-top:0;font-weight:bold;}.pdoc .modulename a{color:var(--link);transition:100ms all;}.pdoc .git-button{float:right;border:solid var(--link) 1px;}.pdoc .git-button:hover{background-color:var(--link);color:var(--pdoc-background);}.view-source-toggle-state,.view-source-toggle-state ~ .pdoc-code{display:none;}.view-source-toggle-state:checked ~ .pdoc-code{display:block;}.view-source-button{display:inline-block;float:right;font-size:.75rem;line-height:1.5rem;color:var(--muted);padding:0 .4rem 0 1.3rem;cursor:pointer;text-indent:-2px;}.view-source-button > span{visibility:hidden;}.module-info .view-source-button{float:none;display:flex;justify-content:flex-end;margin:-1.2rem .4rem -.2rem 0;}.view-source-button::before{position:absolute;content:"View Source";display:list-item;list-style-type:disclosure-closed;}.view-source-toggle-state:checked ~ .attr .view-source-button::before,.view-source-toggle-state:checked ~ .view-source-button::before{list-style-type:disclosure-open;}.pdoc .docstring{margin-bottom:1.5rem;}.pdoc section:not(.module-info) .docstring{margin-left:clamp(0rem, 5vw - 2rem, 1rem);}.pdoc .docstring .pdoc-code{margin-left:1em;margin-right:1em;}.pdoc h1:target,.pdoc h2:target,.pdoc h3:target,.pdoc h4:target,.pdoc h5:target,.pdoc h6:target,.pdoc .pdoc-code > pre > span:target{background-color:var(--active);box-shadow:-1rem 0 0 0 var(--active);}.pdoc .pdoc-code > pre > span:target{display:block;}.pdoc div:target > .attr,.pdoc section:target > .attr,.pdoc dd:target > a{background-color:var(--active);}.pdoc *{scroll-margin:2rem;}.pdoc .pdoc-code .linenos{user-select:none;}.pdoc .attr:hover{filter:contrast(0.95);}.pdoc section, .pdoc .classattr{position:relative;}.pdoc .headerlink{--width:clamp(1rem, 3vw, 2rem);position:absolute;top:0;left:calc(0rem - var(--width));transition:all 100ms ease-in-out;opacity:0;}.pdoc .headerlink::before{content:"#";display:block;text-align:center;width:var(--width);height:2.3rem;line-height:2.3rem;font-size:1.5rem;}.pdoc .attr:hover ~ .headerlink,.pdoc *:target > .headerlink,.pdoc .headerlink:hover{opacity:1;}.pdoc .attr{display:block;margin:.5rem 0 .5rem;padding:.4rem .4rem .4rem 1rem;background-color:var(--accent);overflow-x:auto;}.pdoc .classattr{margin-left:2rem;}.pdoc .name{color:var(--name);font-weight:bold;}.pdoc .def{color:var(--def);font-weight:bold;}.pdoc .signature{background-color:transparent;}.pdoc .param, .pdoc .return-annotation{white-space:pre;}.pdoc .signature.multiline .param{display:block;}.pdoc .signature.condensed .param{display:inline-block;}.pdoc .annotation{color:var(--annotation);}.pdoc .view-value-toggle-state,.pdoc .view-value-toggle-state ~ .default_value{display:none;}.pdoc .view-value-toggle-state:checked ~ .default_value{display:inherit;}.pdoc .view-value-button{font-size:.5rem;vertical-align:middle;border-style:dashed;margin-top:-0.1rem;}.pdoc .view-value-button:hover{background:white;}.pdoc .view-value-button::before{content:"show";text-align:center;width:2.2em;display:inline-block;}.pdoc .view-value-toggle-state:checked ~ .view-value-button::before{content:"hide";}.pdoc .inherited{margin-left:2rem;}.pdoc .inherited dt{font-weight:700;}.pdoc .inherited dt, .pdoc .inherited dd{display:inline;margin-left:0;margin-bottom:.5rem;}.pdoc .inherited dd:not(:last-child):after{content:", ";}.pdoc .inherited .class:before{content:"class ";}.pdoc .inherited .function a:after{content:"()";}.pdoc .search-result .docstring{overflow:auto;max-height:25vh;}.pdoc .search-result.focused > .attr{background-color:var(--active);}.pdoc .attribution{margin-top:2rem;display:block;opacity:0.5;transition:all 200ms;filter:grayscale(100%);}.pdoc .attribution:hover{opacity:1;filter:grayscale(0%);}.pdoc .attribution img{margin-left:5px;height:35px;vertical-align:middle;width:70px;transition:all 200ms;}.pdoc table{display:block;width:max-content;max-width:100%;overflow:auto;margin-bottom:1rem;}.pdoc table th{font-weight:600;}.pdoc table th, .pdoc table td{padding:6px 13px;border:1px solid var(--accent2);}</style></div>