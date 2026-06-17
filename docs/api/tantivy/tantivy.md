---
title: tantivy.tantivy
---

# tantivy

<main class="pdoc">
<section class="module-info">
                    

                        <div class="docstring"><p>Python bindings for the search engine library Tantivy.</p>

<p>Tantivy is a full text search engine library written in rust.</p>

<p>It is closer to Apache Lucene than to Elasticsearch and Apache Solr in
the sense it is not an off-the-shelf search engine server, but rather
a library that can be used to build such a search engine.
Tantivy is, in fact, strongly inspired by Lucene's design.</p>

<h6 id="example">Example:</h6>

<blockquote>
  <div class="pdoc-code codehilite">
<pre><span></span><code><span class="gp">&gt;&gt;&gt; </span><span class="kn">import</span><span class="w"> </span><span class="nn">json</span>
<span class="gp">&gt;&gt;&gt; </span><span class="kn">import</span><span class="w"> </span><span class="nn">tantivy</span>
<span class="gp">&gt;&gt;&gt; </span><span class="n">builder</span> <span class="o">=</span> <span class="n"><a href="#SchemaBuilder">tantivy.SchemaBuilder</a></span><span class="p">()</span>
<span class="gp">&gt;&gt;&gt; </span><span class="n">title</span> <span class="o">=</span> <span class="n">builder</span><span class="o">.</span><span class="n">add_text_field</span><span class="p">(</span><span class="s2">&quot;title&quot;</span><span class="p">,</span> <span class="n">stored</span><span class="o">=</span><span class="kc">True</span><span class="p">)</span>
<span class="gp">&gt;&gt;&gt; </span><span class="n">body</span> <span class="o">=</span> <span class="n">builder</span><span class="o">.</span><span class="n">add_text_field</span><span class="p">(</span><span class="s2">&quot;body&quot;</span><span class="p">)</span>
<span class="gp">&gt;&gt;&gt; </span><span class="n">schema</span> <span class="o">=</span> <span class="n">builder</span><span class="o">.</span><span class="n">build</span><span class="p">()</span>
<span class="gp">&gt;&gt;&gt; </span><span class="n">index</span> <span class="o">=</span> <span class="n"><a href="#Index">tantivy.Index</a></span><span class="p">(</span><span class="n">schema</span><span class="p">)</span>
<span class="gp">&gt;&gt;&gt; </span><span class="n">doc</span> <span class="o">=</span> <span class="n"><a href="#Document">tantivy.Document</a></span><span class="p">()</span>
<span class="gp">&gt;&gt;&gt; </span><span class="n">doc</span><span class="o">.</span><span class="n">add_text</span><span class="p">(</span><span class="n">title</span><span class="p">,</span> <span class="s2">&quot;The Old Man and the Sea&quot;</span><span class="p">)</span>
<span class="gp">&gt;&gt;&gt; </span><span class="n">doc</span><span class="o">.</span><span class="n">add_text</span><span class="p">(</span><span class="n">body</span><span class="p">,</span> <span class="p">(</span><span class="s2">&quot;He was an old man who fished alone in a &quot;</span>
<span class="go">                        &quot;skiff in the Gulf Stream and he had gone &quot;</span>
<span class="go">                        &quot;eighty-four days now without taking a fish.&quot;))</span>
<span class="gp">&gt;&gt;&gt; </span><span class="n">writer</span><span class="o">.</span><span class="n">add_document</span><span class="p">(</span><span class="n">doc</span><span class="p">)</span>
<span class="gp">&gt;&gt;&gt; </span><span class="n">doc</span> <span class="o">=</span> <span class="n">schema</span><span class="o">.</span><span class="n">parse_document</span><span class="p">(</span><span class="n">json</span><span class="o">.</span><span class="n">dumps</span><span class="p">({</span>
<span class="go">       &quot;title&quot;: [&quot;Frankenstein&quot;, &quot;The Modern Prometheus&quot;],</span>
<span class="go">       &quot;body&quot;: (&quot;You will rejoice to hear that no disaster has &quot;</span>
<span class="go">                &quot;accompanied the commencement of an enterprise which &quot;</span>
<span class="go">                &quot;you have regarded with such evil forebodings.  &quot;</span>
<span class="go">                &quot;I arrived here yesterday, and my first task is to &quot;</span>
<span class="go">                &quot;assure my dear sister of my welfare and increasing &quot;</span>
<span class="go">                &quot;confidence in the success of my undertaking.&quot;)</span>
<span class="go">}))</span>
<span class="gp">&gt;&gt;&gt; </span><span class="n">writer</span><span class="o">.</span><span class="n">add_document</span><span class="p">(</span><span class="n">doc</span><span class="p">)</span>
<span class="gp">&gt;&gt;&gt; </span><span class="n">writer</span><span class="o">.</span><span class="n">commit</span><span class="p">()</span>
<span class="gp">&gt;&gt;&gt; </span><span class="n">reader</span> <span class="o">=</span> <span class="n">index</span><span class="o">.</span><span class="n">reader</span><span class="p">()</span>
<span class="gp">&gt;&gt;&gt; </span><span class="n">searcher</span> <span class="o">=</span> <span class="n">reader</span><span class="o">.</span><span class="n">searcher</span><span class="p">()</span>
<span class="gp">&gt;&gt;&gt; </span><span class="n">query</span> <span class="o">=</span> <span class="n">index</span><span class="o">.</span><span class="n">parse_query</span><span class="p">(</span><span class="s2">&quot;sea whale&quot;</span><span class="p">,</span> <span class="p">[</span><span class="n">title</span><span class="p">,</span> <span class="n">body</span><span class="p">])</span>
<span class="gp">&gt;&gt;&gt; </span><span class="n">result</span> <span class="o">=</span> <span class="n">searcher</span><span class="o">.</span><span class="n">search</span><span class="p">(</span><span class="n">query</span><span class="p">,</span> <span class="mi">10</span><span class="p">)</span>
<span class="gp">&gt;&gt;&gt; </span><span class="k">assert</span> <span class="nb">len</span><span class="p">(</span><span class="n">result</span><span class="p">)</span> <span class="o">==</span> <span class="mi">1</span>
</code></pre>
  </div>
</blockquote>
</div>

                
                
                
            </section>
</main>

## __version__

<main class="pdoc">
<section id="__version__">
                    <div class="attr variable">
            <span class="name">__version__</span><span class="annotation">: str</span>        =
<span class="default_value">&#39;tantivy v0.26.0, index_format v7&#39;</span>

        
    </div>
    <a class="headerlink" href="#__version__"></a>
    
    

                </section>
</main>

## DocAddress

<main class="pdoc">
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


                            <div id="DocAddress.doc" class="classattr">
                                <div class="attr variable">
            <span class="name">doc</span><span class="annotation">: int</span>

        
    </div>
    <a class="headerlink" href="#DocAddress.doc"></a>
    
            <div class="docstring"><p>The segment local DocId</p>
</div>


                            </div>
                            <div id="DocAddress.segment_ord" class="classattr">
                                <div class="attr variable">
            <span class="name">segment_ord</span><span class="annotation">: int</span>

        
    </div>
    <a class="headerlink" href="#DocAddress.segment_ord"></a>
    
            <div class="docstring"><p>The segment ordinal is an id identifying the segment hosting the
document. It is only meaningful, in the context of a searcher.</p>
</div>


                            </div>
                </section>
</main>

## Document

<main class="pdoc">
<section id="Document">
                    <div class="attr class">
            
    <span class="def">class</span>
    <span class="name">Document</span>:

        
    </div>
    <a class="headerlink" href="#Document"></a>
    
            <div class="docstring"><p>Tantivy's Document is the object that can be indexed and then searched for.</p>

<p>Documents are fundamentally a collection of unordered tuples
(field_name, value). In this list, one field may appear more than once.</p>

<h6 id="example">Example:</h6>

<blockquote>
  <div class="pdoc-code codehilite">
<pre><span></span><code><span class="gp">&gt;&gt;&gt; </span><span class="n">doc</span> <span class="o">=</span> <span class="n"><a href="#Document">tantivy.Document</a></span><span class="p">()</span>
<span class="gp">&gt;&gt;&gt; </span><span class="n">doc</span><span class="o">.</span><span class="n">add_text</span><span class="p">(</span><span class="s2">&quot;title&quot;</span><span class="p">,</span> <span class="s2">&quot;The Old Man and the Sea&quot;</span><span class="p">)</span>
<span class="gp">&gt;&gt;&gt; </span><span class="n">doc</span><span class="o">.</span><span class="n">add_text</span><span class="p">(</span><span class="s2">&quot;body&quot;</span><span class="p">,</span> <span class="p">(</span><span class="s2">&quot;He was an old man who fished alone in a &quot;</span>
<span class="gp">... </span>                      <span class="s2">&quot;skiff in the Gulf Stream and he had gone &quot;</span>
<span class="gp">... </span>                      <span class="s2">&quot;eighty-four days now without taking a fish.&quot;</span><span class="p">))</span>
<span class="gp">&gt;&gt;&gt; </span><span class="n">doc</span>
<span class="go">Document(body=[He was an ],title=[The Old Ma])</span>
</code></pre>
  </div>
</blockquote>

<p>For simplicity, it is also possible to build a <code><a href="#Document">Document</a></code> by passing the field
values directly as constructor arguments.</p>

<h6 id="example-2">Example:</h6>

<blockquote>
  <div class="pdoc-code codehilite">
<pre><span></span><code><span class="gp">&gt;&gt;&gt; </span><span class="n">doc</span> <span class="o">=</span> <span class="n"><a href="#Document">tantivy.Document</a></span><span class="p">(</span><span class="n">title</span><span class="o">=</span><span class="p">[</span><span class="s2">&quot;The Old Man and the Sea&quot;</span><span class="p">],</span> <span class="n">body</span><span class="o">=</span><span class="p">[</span><span class="s2">&quot;...&quot;</span><span class="p">])</span>
</code></pre>
  </div>
</blockquote>

<p>As syntactic sugar, tantivy also allows the user to pass a single values
if there is only one. In other words, the following is also legal.</p>

<h6 id="example-3">Example:</h6>

<blockquote>
  <div class="pdoc-code codehilite">
<pre><span></span><code><span class="gp">&gt;&gt;&gt; </span><span class="n">doc</span> <span class="o">=</span> <span class="n"><a href="#Document">tantivy.Document</a></span><span class="p">(</span><span class="n">title</span><span class="o">=</span><span class="s2">&quot;The Old Man and the Sea&quot;</span><span class="p">,</span> <span class="n">body</span><span class="o">=</span><span class="s2">&quot;...&quot;</span><span class="p">)</span>
</code></pre>
  </div>
</blockquote>

<p>For numeric fields, the [<code><a href="#Document">Document</a></code>] constructor does not have any
information about the type and will try to guess the type.
Therefore, it is recommended to use the [<code>Document::from_dict()</code>],
[<code>Document::extract()</code>], or <code>Document::add_*()</code> functions to provide
explicit type information.</p>

<h6 id="example-4">Example:</h6>

<blockquote>
  <div class="pdoc-code codehilite">
<pre><span></span><code><span class="gp">&gt;&gt;&gt; </span><span class="n">schema</span> <span class="o">=</span> <span class="p">(</span>
<span class="gp">... </span>    <span class="n">SchemaBuilder</span><span class="p">()</span>
<span class="gp">... </span>        <span class="o">.</span><span class="n">add_unsigned_field</span><span class="p">(</span><span class="s2">&quot;unsigned&quot;</span><span class="p">)</span>
<span class="gp">... </span>        <span class="o">.</span><span class="n">add_integer_field</span><span class="p">(</span><span class="s2">&quot;signed&quot;</span><span class="p">)</span>
<span class="gp">... </span>        <span class="o">.</span><span class="n">add_float_field</span><span class="p">(</span><span class="s2">&quot;float&quot;</span><span class="p">)</span>
<span class="gp">... </span>        <span class="o">.</span><span class="n">build</span><span class="p">()</span>
<span class="gp">... </span><span class="p">)</span>
<span class="gp">&gt;&gt;&gt; </span><span class="n">doc</span> <span class="o">=</span> <span class="n"><a href="#Document.from_dict">tantivy.Document.from_dict</a></span><span class="p">(</span>
<span class="gp">... </span>    <span class="p">{</span><span class="s2">&quot;unsigned&quot;</span><span class="p">:</span> <span class="mi">1000</span><span class="p">,</span> <span class="s2">&quot;signed&quot;</span><span class="p">:</span> <span class="o">-</span><span class="mi">5</span><span class="p">,</span> <span class="s2">&quot;float&quot;</span><span class="p">:</span> <span class="mf">0.4</span><span class="p">},</span>
<span class="gp">... </span>    <span class="n">schema</span><span class="p">,</span>
<span class="gp">... </span><span class="p">)</span>
</code></pre>
  </div>
</blockquote>
</div>


                            <div id="Document.add_boolean" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">add_boolean</span><span class="signature pdoc-code condensed">(<span class="param"><span class="bp">self</span>, </span><span class="param"><span class="n">field_name</span><span class="p">:</span> <span class="nb">str</span>, </span><span class="param"><span class="n">value</span><span class="p">:</span> <span class="nb">bool</span></span><span class="return-annotation">) -> <span class="kc">None</span>:</span></span>

        
    </div>
    <a class="headerlink" href="#Document.add_boolean"></a>
    
            <div class="docstring"><p>Add a boolean value to the document.</p>

<h6 id="arguments">Arguments:</h6>

<ul>
<li><strong>field_name (str):</strong>  The field name for which we are adding the value.</li>
<li><strong>value (bool):</strong>  The boolean that will be added to the document.</li>
</ul>
</div>


                            </div>
                            <div id="Document.add_bytes" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">add_bytes</span><span class="signature pdoc-code condensed">(<span class="param"><span class="bp">self</span>, </span><span class="param"><span class="n">field_name</span><span class="p">:</span> <span class="nb">str</span>, </span><span class="param"><span class="nb">bytes</span><span class="p">:</span> <span class="nb">bytes</span></span><span class="return-annotation">) -> <span class="kc">None</span>:</span></span>

        
    </div>
    <a class="headerlink" href="#Document.add_bytes"></a>
    
            <div class="docstring"><p>Add a bytes value to the document.</p>

<h6 id="arguments">Arguments:</h6>

<ul>
<li><strong>field_name (str):</strong>  The field for which we are adding the bytes.</li>
<li><strong>value (bytes):</strong>  The bytes that will be added to the document.</li>
</ul>
</div>


                            </div>
                            <div id="Document.add_date" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">add_date</span><span class="signature pdoc-code condensed">(<span class="param"><span class="bp">self</span>, </span><span class="param"><span class="n">field_name</span><span class="p">:</span> <span class="nb">str</span>, </span><span class="param"><span class="n">value</span><span class="p">:</span> <span class="n">datetime</span><span class="o">.</span><span class="n">datetime</span></span><span class="return-annotation">) -> <span class="kc">None</span>:</span></span>

        
    </div>
    <a class="headerlink" href="#Document.add_date"></a>
    
            <div class="docstring"><p>Add a date value to the document.</p>

<h6 id="arguments">Arguments:</h6>

<ul>
<li><strong>field_name (str):</strong>  The field name for which we are adding the date.</li>
<li><strong>value (datetime):</strong>  The date that will be added to the document.</li>
</ul>
</div>


                            </div>
                            <div id="Document.add_facet" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">add_facet</span><span class="signature pdoc-code condensed">(<span class="param"><span class="bp">self</span>, </span><span class="param"><span class="n">field_name</span><span class="p">:</span> <span class="nb">str</span>, </span><span class="param"><span class="n">facet</span><span class="p">:</span> <span class="n"><a href="#Facet">Facet</a></span></span><span class="return-annotation">) -> <span class="kc">None</span>:</span></span>

        
    </div>
    <a class="headerlink" href="#Document.add_facet"></a>
    
            <div class="docstring"><p>Add a facet value to the document.</p>

<h6 id="arguments">Arguments:</h6>

<ul>
<li><strong>field_name (str):</strong>  The field name for which we are adding the facet.</li>
<li><strong>value (Facet):</strong>  The Facet that will be added to the document.</li>
</ul>
</div>


                            </div>
                            <div id="Document.add_float" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">add_float</span><span class="signature pdoc-code condensed">(<span class="param"><span class="bp">self</span>, </span><span class="param"><span class="n">field_name</span><span class="p">:</span> <span class="nb">str</span>, </span><span class="param"><span class="n">value</span><span class="p">:</span> <span class="nb">float</span></span><span class="return-annotation">) -> <span class="kc">None</span>:</span></span>

        
    </div>
    <a class="headerlink" href="#Document.add_float"></a>
    
            <div class="docstring"><p>Add a float value to the document.</p>

<h6 id="arguments">Arguments:</h6>

<ul>
<li><strong>field_name (str):</strong>  The field name for which we are adding the value.</li>
<li><strong>value (f64):</strong>  The float that will be added to the document.</li>
</ul>
</div>


                            </div>
                            <div id="Document.add_integer" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">add_integer</span><span class="signature pdoc-code condensed">(<span class="param"><span class="bp">self</span>, </span><span class="param"><span class="n">field_name</span><span class="p">:</span> <span class="nb">str</span>, </span><span class="param"><span class="n">value</span><span class="p">:</span> <span class="nb">int</span></span><span class="return-annotation">) -> <span class="kc">None</span>:</span></span>

        
    </div>
    <a class="headerlink" href="#Document.add_integer"></a>
    
            <div class="docstring"><p>Add a signed integer value to the document.</p>

<h6 id="arguments">Arguments:</h6>

<ul>
<li><strong>field_name (str):</strong>  The field name for which we are adding the integer.</li>
<li><strong>value (int):</strong>  The integer that will be added to the document.</li>
</ul>
</div>


                            </div>
                            <div id="Document.add_ip_addr" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">add_ip_addr</span><span class="signature pdoc-code condensed">(<span class="param"><span class="bp">self</span>, </span><span class="param"><span class="n">field_name</span><span class="p">:</span> <span class="nb">str</span>, </span><span class="param"><span class="n">ip_addr</span><span class="p">:</span> <span class="nb">str</span></span><span class="return-annotation">) -> <span class="kc">None</span>:</span></span>

        
    </div>
    <a class="headerlink" href="#Document.add_ip_addr"></a>
    
            <div class="docstring"><p>Add an IP address value to the document.</p>

<h6 id="arguments">Arguments:</h6>

<ul>
<li><strong>field_name (str):</strong>  The field for which we are adding the IP address.</li>
<li><strong>value (str):</strong>  The IP address object that will be added
to the document.</li>
</ul>

<p>Raises a ValueError if the IP address is invalid.</p>
</div>


                            </div>
                            <div id="Document.add_json" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">add_json</span><span class="signature pdoc-code condensed">(<span class="param"><span class="bp">self</span>, </span><span class="param"><span class="n">field_name</span><span class="p">:</span> <span class="nb">str</span>, </span><span class="param"><span class="n">value</span><span class="p">:</span> <span class="n">Any</span></span><span class="return-annotation">) -> <span class="kc">None</span>:</span></span>

        
    </div>
    <a class="headerlink" href="#Document.add_json"></a>
    
            <div class="docstring"><p>Add a JSON value to the document.</p>

<h6 id="arguments">Arguments:</h6>

<ul>
<li><strong>field_name (str):</strong>  The field for which we are adding the JSON.</li>
<li><strong>value (str | Dict[str, Any]):</strong>  The JSON object that will be added
to the document.</li>
</ul>

<p>Raises a ValueError if the JSON is invalid.</p>
</div>


                            </div>
                            <div id="Document.add_text" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">add_text</span><span class="signature pdoc-code condensed">(<span class="param"><span class="bp">self</span>, </span><span class="param"><span class="n">field_name</span><span class="p">:</span> <span class="nb">str</span>, </span><span class="param"><span class="n">text</span><span class="p">:</span> <span class="nb">str</span></span><span class="return-annotation">) -> <span class="kc">None</span>:</span></span>

        
    </div>
    <a class="headerlink" href="#Document.add_text"></a>
    
            <div class="docstring"><p>Add a text value to the document.</p>

<h6 id="arguments">Arguments:</h6>

<ul>
<li><strong>field_name (str):</strong>  The field name for which we are adding the text.</li>
<li><strong>text (str):</strong>  The text that will be added to the document.</li>
</ul>
</div>


                            </div>
                            <div id="Document.add_unsigned" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">add_unsigned</span><span class="signature pdoc-code condensed">(<span class="param"><span class="bp">self</span>, </span><span class="param"><span class="n">field_name</span><span class="p">:</span> <span class="nb">str</span>, </span><span class="param"><span class="n">value</span><span class="p">:</span> <span class="nb">int</span></span><span class="return-annotation">) -> <span class="kc">None</span>:</span></span>

        
    </div>
    <a class="headerlink" href="#Document.add_unsigned"></a>
    
            <div class="docstring"><p>Add an unsigned integer value to the document.</p>

<h6 id="arguments">Arguments:</h6>

<ul>
<li><strong>field_name (str):</strong>  The field name for which we are adding the unsigned integer.</li>
<li><strong>value (int):</strong>  The integer that will be added to the document.</li>
</ul>
</div>


                            </div>
                            <div id="Document.extend" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">extend</span><span class="signature pdoc-code condensed">(<span class="param"><span class="bp">self</span>, </span><span class="param"><span class="n">py_dict</span><span class="p">:</span> <span class="nb">dict</span>, </span><span class="param"><span class="n">schema</span><span class="p">:</span> <span class="n"><a href="#Schema">Schema</a></span> <span class="o">|</span> <span class="kc">None</span></span><span class="return-annotation">) -> <span class="kc">None</span>:</span></span>

        
    </div>
    <a class="headerlink" href="#Document.extend"></a>
    
            <div class="docstring"><p>The type of the None singleton.</p>
</div>


                            </div>
                            <div id="Document.from_dict" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">from_dict</span><span class="signature pdoc-code multiline">(<span class="param">    <span class="n">py_dict</span><span class="p">:</span> <span class="nb">dict</span>,</span><span class="param">    <span class="n">schema</span><span class="p">:</span> <span class="n"><a href="#Schema">Schema</a></span> <span class="o">|</span> <span class="kc">None</span> <span class="o">=</span> <span class="kc">None</span></span><span class="return-annotation">) -> <span class="n"><a href="#Document">Document</a></span>:</span></span>

        
    </div>
    <a class="headerlink" href="#Document.from_dict"></a>
    
            <div class="docstring"><p>The type of the None singleton.</p>
</div>


                            </div>
                            <div id="Document.get_all" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">get_all</span><span class="signature pdoc-code condensed">(<span class="param"><span class="bp">self</span>, </span><span class="param"><span class="n">field_name</span><span class="p">:</span> <span class="nb">str</span></span><span class="return-annotation">) -> <span class="nb">list</span><span class="p">[</span><span class="n">typing</span><span class="o">.</span><span class="n">Any</span><span class="p">]</span>:</span></span>

        
    </div>
    <a class="headerlink" href="#Document.get_all"></a>
    
            <div class="docstring"><p>Get the all values associated with the given field.</p>

<h6 id="arguments">Arguments:</h6>

<ul>
<li><strong>field (Field):</strong>  The field for which we would like to get the values.</li>
</ul>

<p>Returns a list of values.
The type of the value depends on the field.</p>
</div>


                            </div>
                            <div id="Document.get_first" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">get_first</span><span class="signature pdoc-code condensed">(<span class="param"><span class="bp">self</span>, </span><span class="param"><span class="n">field_name</span><span class="p">:</span> <span class="nb">str</span></span><span class="return-annotation">) -> <span class="n">Any</span> <span class="o">|</span> <span class="kc">None</span>:</span></span>

        
    </div>
    <a class="headerlink" href="#Document.get_first"></a>
    
            <div class="docstring"><p>Get the first value associated with the given field.</p>

<h6 id="arguments">Arguments:</h6>

<ul>
<li><strong>field (Field):</strong>  The field for which we would like to get the value.</li>
</ul>

<p>Returns the value if one is found, otherwise None.
The type of the value depends on the field.</p>
</div>


                            </div>
                            <div id="Document.is_empty" class="classattr">
                                <div class="attr variable">
            <span class="name">is_empty</span><span class="annotation">: bool</span>

        
    </div>
    <a class="headerlink" href="#Document.is_empty"></a>
    
            <div class="docstring"><p>True if the document is empty, False otherwise.</p>
</div>


                            </div>
                            <div id="Document.num_fields" class="classattr">
                                <div class="attr variable">
            <span class="name">num_fields</span><span class="annotation">: int</span>

        
    </div>
    <a class="headerlink" href="#Document.num_fields"></a>
    
            <div class="docstring"><p>Returns the number of added fields that have been added to the document</p>
</div>


                            </div>
                            <div id="Document.to_dict" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">to_dict</span><span class="signature pdoc-code condensed">(<span class="param"><span class="bp">self</span></span><span class="return-annotation">) -> <span class="nb">dict</span><span class="p">[</span><span class="nb">str</span><span class="p">,</span> <span class="nb">list</span><span class="p">[</span><span class="n">typing</span><span class="o">.</span><span class="n">Any</span><span class="p">]]</span>:</span></span>

        
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
                </section>
</main>

## Explanation

<main class="pdoc">
<section id="Explanation">
                    <div class="attr class">
            
    <span class="def">class</span>
    <span class="name">Explanation</span>:

        
    </div>
    <a class="headerlink" href="#Explanation"></a>
    
            <div class="docstring"><p>Represents an explanation of how a document matched a query.</p>
</div>


                            <div id="Explanation.to_json" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">to_json</span><span class="signature pdoc-code condensed">(<span class="param"><span class="bp">self</span></span><span class="return-annotation">) -> <span class="nb">str</span>:</span></span>

        
    </div>
    <a class="headerlink" href="#Explanation.to_json"></a>
    
            <div class="docstring"><p>Returns a JSON representation of the explanation.</p>
</div>


                            </div>
                </section>
</main>

## Facet

<main class="pdoc">
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
        <span class="name">from_encoded</span><span class="signature pdoc-code condensed">(<span class="param"><span class="n">encoded_bytes</span><span class="p">:</span> <span class="nb">bytes</span></span><span class="return-annotation">) -> <span class="n"><a href="#Facet">Facet</a></span>:</span></span>

        
    </div>
    <a class="headerlink" href="#Facet.from_encoded"></a>
    
            <div class="docstring"><p>Creates a <code><a href="#Facet">Facet</a></code> from its binary representation.</p>
</div>


                            </div>
                            <div id="Facet.from_string" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">from_string</span><span class="signature pdoc-code condensed">(<span class="param"><span class="bp">cls</span>, </span><span class="param"><span class="n">facet_string</span><span class="p">:</span> <span class="nb">str</span></span><span class="return-annotation">) -> <span class="n"><a href="#Facet">Facet</a></span>:</span></span>

        
    </div>
    <a class="headerlink" href="#Facet.from_string"></a>
    
            <div class="docstring"><p>Create a Facet object from a string.</p>

<h6 id="arguments">Arguments:</h6>

<ul>
<li><strong>facet_string (str):</strong>  The string that contains a facet.</li>
</ul>

<p>Returns the created Facet.</p>
</div>


                            </div>
                            <div id="Facet.is_prefix_of" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">is_prefix_of</span><span class="signature pdoc-code condensed">(<span class="param"><span class="bp">self</span>, </span><span class="param"><span class="n">other</span><span class="p">:</span> <span class="n"><a href="#Facet">Facet</a></span></span><span class="return-annotation">) -> <span class="nb">bool</span>:</span></span>

        
    </div>
    <a class="headerlink" href="#Facet.is_prefix_of"></a>
    
            <div class="docstring"><p>Returns true if another Facet is a subfacet of this facet.</p>

<h6 id="arguments">Arguments:</h6>

<ul>
<li><strong>other (Facet):</strong>  The Facet that we should check if this facet is a
subset of.</li>
</ul>
</div>


                            </div>
                            <div id="Facet.is_root" class="classattr">
                                <div class="attr variable">
            <span class="name">is_root</span><span class="annotation">: bool</span>

        
    </div>
    <a class="headerlink" href="#Facet.is_root"></a>
    
            <div class="docstring"><p>Returns true if the facet is the root facet /.</p>
</div>


                            </div>
                            <div id="Facet.root" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">root</span><span class="signature pdoc-code condensed">(<span class="param"><span class="bp">cls</span></span><span class="return-annotation">) -> <span class="n"><a href="#Facet">Facet</a></span>:</span></span>

        
    </div>
    <a class="headerlink" href="#Facet.root"></a>
    
            <div class="docstring"><p>Create a new instance of the "root facet" Equivalent to /.</p>
</div>


                            </div>
                            <div id="Facet.to_path" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">to_path</span><span class="signature pdoc-code condensed">(<span class="param"><span class="bp">self</span></span><span class="return-annotation">) -> <span class="nb">list</span><span class="p">[</span><span class="nb">str</span><span class="p">]</span>:</span></span>

        
    </div>
    <a class="headerlink" href="#Facet.to_path"></a>
    
            <div class="docstring"><p>Returns the list of <code>segments</code> that forms a facet path.</p>

<p>For instance <code>//europe/france</code> becomes <code>["europe", "france"]</code>.</p>
</div>


                            </div>
                            <div id="Facet.to_path_str" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">to_path_str</span><span class="signature pdoc-code condensed">(<span class="param"><span class="bp">self</span></span><span class="return-annotation">) -> <span class="nb">str</span>:</span></span>

        
    </div>
    <a class="headerlink" href="#Facet.to_path_str"></a>
    
            <div class="docstring"><p>Returns the facet string representation.</p>
</div>


                            </div>
                </section>
</main>

## FieldType

<main class="pdoc">
<section id="FieldType">
                    <div class="attr class">
            
    <span class="def">class</span>
    <span class="name">FieldType</span>:

        
    </div>
    <a class="headerlink" href="#FieldType"></a>
    
            <div class="docstring"><p>Tantivy's Type</p>
</div>


                            <div id="FieldType.Boolean" class="classattr">
                                <div class="attr variable">
            <span class="name">Boolean</span>        =
<span class="default_value"><a href="#FieldType.Boolean">FieldType.Boolean</a></span>

        
    </div>
    <a class="headerlink" href="#FieldType.Boolean"></a>
    
    

                            </div>
                            <div id="FieldType.Bytes" class="classattr">
                                <div class="attr variable">
            <span class="name">Bytes</span>        =
<span class="default_value"><a href="#FieldType.Bytes">FieldType.Bytes</a></span>

        
    </div>
    <a class="headerlink" href="#FieldType.Bytes"></a>
    
    

                            </div>
                            <div id="FieldType.Date" class="classattr">
                                <div class="attr variable">
            <span class="name">Date</span>        =
<span class="default_value"><a href="#FieldType.Date">FieldType.Date</a></span>

        
    </div>
    <a class="headerlink" href="#FieldType.Date"></a>
    
    

                            </div>
                            <div id="FieldType.Facet" class="classattr">
                                <div class="attr variable">
            <span class="name">Facet</span>        =
<span class="default_value"><a href="#FieldType.Facet">FieldType.Facet</a></span>

        
    </div>
    <a class="headerlink" href="#FieldType.Facet"></a>
    
    

                            </div>
                            <div id="FieldType.Float" class="classattr">
                                <div class="attr variable">
            <span class="name">Float</span>        =
<span class="default_value"><a href="#FieldType.Float">FieldType.Float</a></span>

        
    </div>
    <a class="headerlink" href="#FieldType.Float"></a>
    
    

                            </div>
                            <div id="FieldType.Integer" class="classattr">
                                <div class="attr variable">
            <span class="name">Integer</span>        =
<span class="default_value"><a href="#FieldType.Integer">FieldType.Integer</a></span>

        
    </div>
    <a class="headerlink" href="#FieldType.Integer"></a>
    
    

                            </div>
                            <div id="FieldType.IpAddr" class="classattr">
                                <div class="attr variable">
            <span class="name">IpAddr</span>        =
<span class="default_value"><a href="#FieldType.IpAddr">FieldType.IpAddr</a></span>

        
    </div>
    <a class="headerlink" href="#FieldType.IpAddr"></a>
    
    

                            </div>
                            <div id="FieldType.Json" class="classattr">
                                <div class="attr variable">
            <span class="name">Json</span>        =
<span class="default_value"><a href="#FieldType.Json">FieldType.Json</a></span>

        
    </div>
    <a class="headerlink" href="#FieldType.Json"></a>
    
    

                            </div>
                            <div id="FieldType.Text" class="classattr">
                                <div class="attr variable">
            <span class="name">Text</span>        =
<span class="default_value"><a href="#FieldType.Text">FieldType.Text</a></span>

        
    </div>
    <a class="headerlink" href="#FieldType.Text"></a>
    
    

                            </div>
                            <div id="FieldType.Unsigned" class="classattr">
                                <div class="attr variable">
            <span class="name">Unsigned</span>        =
<span class="default_value"><a href="#FieldType.Unsigned">FieldType.Unsigned</a></span>

        
    </div>
    <a class="headerlink" href="#FieldType.Unsigned"></a>
    
    

                            </div>
                </section>
</main>

## Filter

<main class="pdoc">
<section id="Filter">
                    <div class="attr class">
            
    <span class="def">class</span>
    <span class="name">Filter</span>:

        
    </div>
    <a class="headerlink" href="#Filter"></a>
    
            <div class="docstring"><p>All Tantivy's builtin TokenFilters.</p>

<h2 id="example">Example</h2>

<div class="pdoc-code codehilite">
<pre><span></span><code><span class="nb">filter</span> <span class="o">=</span> <span class="n">Filter</span><span class="o">.</span><span class="n">alpha_num</span><span class="p">()</span>
</code></pre>
</div>

<h2 id="usage">Usage</h2>

<p>In general, filter objects exist to
be passed to the filter() method
of a TextAnalyzerBuilder instance.</p>

<p><a href="https://docs.rs/tantivy/latest/tantivy/tokenizer/index.html">https://docs.rs/tantivy/latest/tantivy/tokenizer/index.html</a></p>
</div>


                            <div id="Filter.alphanum_only" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">alphanum_only</span><span class="signature pdoc-code condensed">(<span class="return-annotation">) -> <span class="n"><a href="#Filter">Filter</a></span>:</span></span>

        
    </div>
    <a class="headerlink" href="#Filter.alphanum_only"></a>
    
            <div class="docstring"><p>AlphaNumOnlyFilter</p>
</div>


                            </div>
                            <div id="Filter.ascii_fold" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">ascii_fold</span><span class="signature pdoc-code condensed">(<span class="return-annotation">) -> <span class="n"><a href="#Filter">Filter</a></span>:</span></span>

        
    </div>
    <a class="headerlink" href="#Filter.ascii_fold"></a>
    
            <div class="docstring"><p>AsciiFoldingFilter</p>
</div>


                            </div>
                            <div id="Filter.custom_stopword" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">custom_stopword</span><span class="signature pdoc-code condensed">(<span class="param"><span class="n">stopwords</span><span class="p">:</span> <span class="nb">list</span><span class="p">[</span><span class="nb">str</span><span class="p">]</span></span><span class="return-annotation">) -> <span class="n"><a href="#Filter">Filter</a></span>:</span></span>

        
    </div>
    <a class="headerlink" href="#Filter.custom_stopword"></a>
    
            <div class="docstring"><p>StopWordFilter (user-provided stop word list)</p>

<p>This variant of <a href="#Filter.stopword">Filter.stopword()</a> lets you provide
your own custom list of stopwords.</p>

<p>Args:</p>

<ul>
<li>stopwords (list(str)): a list of words to be removed.</li>
</ul>
</div>


                            </div>
                            <div id="Filter.lowercase" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">lowercase</span><span class="signature pdoc-code condensed">(<span class="return-annotation">) -> <span class="n"><a href="#Filter">Filter</a></span>:</span></span>

        
    </div>
    <a class="headerlink" href="#Filter.lowercase"></a>
    
            <div class="docstring"><p>The type of the None singleton.</p>
</div>


                            </div>
                            <div id="Filter.remove_long" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">remove_long</span><span class="signature pdoc-code condensed">(<span class="param"><span class="n">length_limit</span><span class="p">:</span> <span class="nb">int</span></span><span class="return-annotation">) -> <span class="n"><a href="#Filter">Filter</a></span>:</span></span>

        
    </div>
    <a class="headerlink" href="#Filter.remove_long"></a>
    
            <div class="docstring"><p>RemoveLongFilter</p>

<p>Args:</p>

<ul>
<li>length_limit (int): max character length of token.</li>
</ul>
</div>


                            </div>
                            <div id="Filter.split_compound" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">split_compound</span><span class="signature pdoc-code condensed">(<span class="param"><span class="n">constituent_words</span><span class="p">:</span> <span class="nb">list</span><span class="p">[</span><span class="nb">str</span><span class="p">]</span></span><span class="return-annotation">) -> <span class="n"><a href="#Filter">Filter</a></span>:</span></span>

        
    </div>
    <a class="headerlink" href="#Filter.split_compound"></a>
    
            <div class="docstring"><p>SplitCompoundWords</p>

<p><a href="https://docs.rs/tantivy/latest/tantivy/tokenizer/struct.SplitCompoundWords.html">https://docs.rs/tantivy/latest/tantivy/tokenizer/struct.SplitCompoundWords.html</a></p>

<p>Args:</p>

<ul>
<li>constituent_words (list(string)): words that make up compound word (must be in order).</li>
</ul>

<p>Example:</p>

<div class="pdoc-code codehilite">
<pre><span></span><code><span class="c1"># useless, contrived example:</span>
<span class="n">compound_spliter</span> <span class="o">=</span> <span class="n">Filter</span><span class="o">.</span><span class="n">split_compounds</span><span class="p">([</span><span class="s1">&#39;butter&#39;</span><span class="p">,</span> <span class="s1">&#39;fly&#39;</span><span class="p">])</span>
<span class="c1"># Will split &#39;butterfly&#39; -&gt; [&#39;butter&#39;, &#39;fly&#39;],</span>
<span class="c1"># but won&#39;t split &#39;buttering&#39; or &#39;buttercupfly&#39;</span>
</code></pre>
</div>
</div>


                            </div>
                            <div id="Filter.stemmer" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">stemmer</span><span class="signature pdoc-code condensed">(<span class="param"><span class="n">language</span><span class="p">:</span> <span class="nb">str</span></span><span class="return-annotation">) -> <span class="n"><a href="#Filter">Filter</a></span>:</span></span>

        
    </div>
    <a class="headerlink" href="#Filter.stemmer"></a>
    
            <div class="docstring"><p>Stemmer</p>
</div>


                            </div>
                            <div id="Filter.stopword" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">stopword</span><span class="signature pdoc-code condensed">(<span class="param"><span class="n">language</span><span class="p">:</span> <span class="nb">str</span></span><span class="return-annotation">) -> <span class="n"><a href="#Filter">Filter</a></span>:</span></span>

        
    </div>
    <a class="headerlink" href="#Filter.stopword"></a>
    
            <div class="docstring"><p>StopWordFilter (builtin stop word list)</p>

<p>Args:</p>

<ul>
<li>language (string): Stop words list language.
Valid values: {
  "arabic", "danish", "dutch", "english", "finnish", "french", "german", "greek",
  "hungarian", "italian", "norwegian", "portuguese", "romanian", "russian",
  "spanish", "swedish", "tamil", "turkish"
}</li>
</ul>
</div>


                            </div>
                </section>
</main>

## Index

<main class="pdoc">
<section id="Index">
                    <div class="attr class">
            
    <span class="def">class</span>
    <span class="name">Index</span>:

        
    </div>
    <a class="headerlink" href="#Index"></a>
    
            <div class="docstring"><p>Create a new index object.</p>

<h6 id="arguments">Arguments:</h6>

<ul>
<li><strong>schema (Schema):</strong>  The schema of the index.</li>
<li><strong>path (str, optional):</strong>  The path where the index should be stored. If
no path is provided, the index will be stored in memory.</li>
<li><strong>reuse (bool, optional):</strong>  Should we open an existing index if one exists
or always create a new one.</li>
</ul>

<p>If an index already exists it will be opened and reused. Raises OSError
if there was a problem during the opening or creation of the index.</p>
</div>


                            <div id="Index.config_reader" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">config_reader</span><span class="signature pdoc-code condensed">(<span class="param"><span class="bp">self</span>, </span><span class="param"><span class="n">reload_policy</span><span class="p">:</span> <span class="nb">str</span> <span class="o">=</span> <span class="s1">&#39;commit&#39;</span>, </span><span class="param"><span class="n">num_warmers</span><span class="p">:</span> <span class="nb">int</span> <span class="o">=</span> <span class="mi">0</span></span><span class="return-annotation">) -> <span class="kc">None</span>:</span></span>

        
    </div>
    <a class="headerlink" href="#Index.config_reader"></a>
    
            <div class="docstring"><p>Configure the index reader.</p>

<h6 id="arguments">Arguments:</h6>

<ul>
<li><strong>reload_policy (str, optional):</strong>  The reload policy that the
IndexReader should use. Can be <code>Manual</code> or <code>OnCommit</code>.</li>
<li><strong>num_warmers (int, optional):</strong>  The number of searchers that the
reader should create.</li>
</ul>
</div>


                            </div>
                            <div id="Index.exists" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">exists</span><span class="signature pdoc-code condensed">(<span class="param"><span class="n">path</span><span class="p">:</span> <span class="nb">str</span></span><span class="return-annotation">) -> <span class="nb">bool</span>:</span></span>

        
    </div>
    <a class="headerlink" href="#Index.exists"></a>
    
            <div class="docstring"><p>Check if the given path contains an existing index.</p>

<h6 id="arguments">Arguments:</h6>

<ul>
<li><strong>path:</strong>  The path where tantivy will search for an index.</li>
</ul>

<p>Returns True if an index exists at the given path, False otherwise.</p>

<p>Raises OSError if the directory cannot be opened.</p>
</div>


                            </div>
                            <div id="Index.is_compatible" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">is_compatible</span><span class="signature pdoc-code condensed">(<span class="param"><span class="n">path</span><span class="p">:</span> <span class="nb">str</span></span><span class="return-annotation">) -> <span class="nb">bool</span>:</span></span>

        
    </div>
    <a class="headerlink" href="#Index.is_compatible"></a>
    
            <div class="docstring"><p>Check whether the index stored at <code>path</code> can be opened by this version
of tantivy.</p>

<p>Tantivy stores the index format version in each segment file. When that
version falls outside the range supported by the installed tantivy, the
index cannot be opened. This method reports that without raising, so a
caller can decide how to handle an incompatible index (for example, by
rebuilding it).</p>

<h6 id="arguments">Arguments:</h6>

<ul>
<li><strong>path (str):</strong>  The directory containing the index.</li>
</ul>

<p>Returns True if the index is compatible, False if it was built with an
unsupported index format version.</p>

<p>Raises ValueError if no index could be found at the given path or if it
could not be read for any other reason.</p>
</div>


                            </div>
                            <div id="Index.open" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">open</span><span class="signature pdoc-code condensed">(<span class="param"><span class="n">path</span><span class="p">:</span> <span class="nb">str</span></span><span class="return-annotation">) -> <span class="n"><a href="#Index">Index</a></span>:</span></span>

        
    </div>
    <a class="headerlink" href="#Index.open"></a>
    
            <div class="docstring"><p>The type of the None singleton.</p>
</div>


                            </div>
                            <div id="Index.parse_query" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">parse_query</span><span class="signature pdoc-code multiline">(<span class="param">    <span class="bp">self</span>,</span><span class="param">    <span class="n">query</span><span class="p">:</span> <span class="nb">str</span>,</span><span class="param">    <span class="n">default_field_names</span><span class="p">:</span> <span class="nb">list</span><span class="p">[</span><span class="nb">str</span><span class="p">]</span> <span class="o">|</span> <span class="kc">None</span> <span class="o">=</span> <span class="kc">None</span>,</span><span class="param">    <span class="n">field_boosts</span><span class="p">:</span> <span class="nb">dict</span><span class="p">[</span><span class="nb">str</span><span class="p">,</span> <span class="nb">float</span><span class="p">]</span> <span class="o">|</span> <span class="kc">None</span> <span class="o">=</span> <span class="kc">None</span>,</span><span class="param">    <span class="n">fuzzy_fields</span><span class="p">:</span> <span class="nb">dict</span><span class="p">[</span><span class="nb">str</span><span class="p">,</span> <span class="nb">tuple</span><span class="p">[</span><span class="nb">bool</span><span class="p">,</span> <span class="nb">int</span><span class="p">,</span> <span class="nb">bool</span><span class="p">]]</span> <span class="o">|</span> <span class="kc">None</span> <span class="o">=</span> <span class="kc">None</span>,</span><span class="param">    <span class="n">conjunction_by_default</span><span class="p">:</span> <span class="nb">bool</span> <span class="o">=</span> <span class="kc">False</span>,</span><span class="param">    <span class="n">allow_regexes</span><span class="p">:</span> <span class="nb">bool</span> <span class="o">=</span> <span class="kc">False</span></span><span class="return-annotation">) -> <span class="n"><a href="#Query">Query</a></span>:</span></span>

        
    </div>
    <a class="headerlink" href="#Index.parse_query"></a>
    
            <div class="docstring"><p>Parse a query</p>

<h6 id="arguments">Arguments:</h6>

<ul>
<li><strong>query:</strong>  the query, following the tantivy query language.</li>
<li><strong>default_fields_names (List[Field]):</strong>  A list of fields used to search if no
field is specified in the query.</li>
<li><strong>field_boosts:</strong>  A dictionary keyed on field names which provides default boosts
for the query constructed by this method.</li>
<li><strong>fuzzy_fields:</strong>  A dictionary keyed on field names which provides (prefix, distance, transpose_cost_one)
triples making queries constructed by this method fuzzy against the given fields
and using the given parameters.
<code>prefix</code> determines if terms which are prefixes of the given term match the query.
<code>distance</code> determines the maximum Levenshtein distance between terms matching the query and the given term.
<code>transpose_cost_one</code> determines if transpositions of neighbouring characters are counted only once against the Levenshtein distance.</li>
<li><strong>conjunction_by_default:</strong>  If true, the query will be parsed as a conjunction query. Defaults to a disjunction query.</li>
<li><strong>allow_regexes:</strong>  If true, allow regexes in queries.</li>
</ul>
</div>


                            </div>
                            <div id="Index.parse_query_lenient" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">parse_query_lenient</span><span class="signature pdoc-code multiline">(<span class="param">    <span class="bp">self</span>,</span><span class="param">    <span class="n">query</span><span class="p">:</span> <span class="nb">str</span>,</span><span class="param">    <span class="n">default_field_names</span><span class="p">:</span> <span class="nb">list</span><span class="p">[</span><span class="nb">str</span><span class="p">]</span> <span class="o">|</span> <span class="kc">None</span> <span class="o">=</span> <span class="kc">None</span>,</span><span class="param">    <span class="n">field_boosts</span><span class="p">:</span> <span class="nb">dict</span><span class="p">[</span><span class="nb">str</span><span class="p">,</span> <span class="nb">float</span><span class="p">]</span> <span class="o">|</span> <span class="kc">None</span> <span class="o">=</span> <span class="kc">None</span>,</span><span class="param">    <span class="n">fuzzy_fields</span><span class="p">:</span> <span class="nb">dict</span><span class="p">[</span><span class="nb">str</span><span class="p">,</span> <span class="nb">tuple</span><span class="p">[</span><span class="nb">bool</span><span class="p">,</span> <span class="nb">int</span><span class="p">,</span> <span class="nb">bool</span><span class="p">]]</span> <span class="o">|</span> <span class="kc">None</span> <span class="o">=</span> <span class="kc">None</span>,</span><span class="param">    <span class="n">conjunction_by_default</span><span class="p">:</span> <span class="nb">bool</span> <span class="o">=</span> <span class="kc">False</span>,</span><span class="param">    <span class="n">allow_regexes</span><span class="p">:</span> <span class="nb">bool</span> <span class="o">=</span> <span class="kc">False</span></span><span class="return-annotation">) -> <span class="nb">tuple</span><span class="p">[</span><span class="n"><a href="#Query">Query</a></span><span class="p">,</span> <span class="nb">list</span><span class="p">[</span><span class="n">typing</span><span class="o">.</span><span class="n">Any</span><span class="p">]]</span>:</span></span>

        
    </div>
    <a class="headerlink" href="#Index.parse_query_lenient"></a>
    
            <div class="docstring"><p>Parse a query leniently.</p>

<p>This variant parses invalid query on a best effort basis. If some part of the query can't
reasonably be executed (range query without field, searching on a non existing field,
searching without precising field when no default field is provided...), they may get turned
into a "match-nothing" subquery.</p>

<h6 id="arguments">Arguments:</h6>

<ul>
<li><strong>query:</strong>  the query, following the tantivy query language.</li>
<li><strong>default_fields_names (List[Field]):</strong>  A list of fields used to search if no
field is specified in the query.</li>
<li><strong>field_boosts:</strong>  A dictionary keyed on field names which provides default boosts
for the query constructed by this method.</li>
<li><strong>fuzzy_fields:</strong>  A dictionary keyed on field names which provides (prefix, distance, transpose_cost_one)
triples making queries constructed by this method fuzzy against the given fields
and using the given parameters.
<code>prefix</code> determines if terms which are prefixes of the given term match the query.
<code>distance</code> determines the maximum Levenshtein distance between terms matching the query and the given term.
<code>transpose_cost_one</code> determines if transpositions of neighbouring characters are counted only once against the Levenshtein distance.</li>
<li><strong>conjunction_by_default:</strong>  If true, the query will be parsed as a conjunction query. Defaults to a disjunction query.</li>
<li><strong>allow_regexes:</strong>  If true, allow regexes in queries.</li>
</ul>

<p>Returns a tuple containing the parsed query and a list of errors.</p>

<p>Raises ValueError if a field in <code>default_field_names</code> is not defined or marked as indexed.</p>
</div>


                            </div>
                            <div id="Index.register_fast_field_tokenizer" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">register_fast_field_tokenizer</span><span class="signature pdoc-code condensed">(<span class="param"><span class="bp">self</span>, </span><span class="param"><span class="n">name</span><span class="p">:</span> <span class="nb">str</span>, </span><span class="param"><span class="n">text_analyzer</span><span class="p">:</span> <span class="n"><a href="#TextAnalyzer">TextAnalyzer</a></span></span><span class="return-annotation">) -> <span class="kc">None</span>:</span></span>

        
    </div>
    <a class="headerlink" href="#Index.register_fast_field_tokenizer"></a>
    
            <div class="docstring"><p>Register a custom text analyzer for fast fields by name. (Confusingly,
this is one of the places where Tantivy uses 'tokenizer' to refer to a
TextAnalyzer instance.)</p>
</div>


                            </div>
                            <div id="Index.register_tokenizer" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">register_tokenizer</span><span class="signature pdoc-code condensed">(<span class="param"><span class="bp">self</span>, </span><span class="param"><span class="n">name</span><span class="p">:</span> <span class="nb">str</span>, </span><span class="param"><span class="n">text_analyzer</span><span class="p">:</span> <span class="n"><a href="#TextAnalyzer">TextAnalyzer</a></span></span><span class="return-annotation">) -> <span class="kc">None</span>:</span></span>

        
    </div>
    <a class="headerlink" href="#Index.register_tokenizer"></a>
    
            <div class="docstring"><p>Register a custom text analyzer by name. (Confusingly,
this is one of the places where Tantivy uses 'tokenizer' to refer to a
TextAnalyzer instance.)</p>
</div>


                            </div>
                            <div id="Index.reload" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">reload</span><span class="signature pdoc-code condensed">(<span class="param"><span class="bp">self</span></span><span class="return-annotation">) -> <span class="kc">None</span>:</span></span>

        
    </div>
    <a class="headerlink" href="#Index.reload"></a>
    
            <div class="docstring"><p>Update searchers so that they reflect the state of the last .commit().</p>

<p>If you set up the the reload policy to be on 'commit' (which is the
default) every commit should be rapidly reflected on your IndexReader
and you should not need to call reload() at all.</p>
</div>


                            </div>
                            <div id="Index.schema" class="classattr">
                                <div class="attr variable">
            <span class="name">schema</span><span class="annotation">: <a href="#Schema">Schema</a></span>

        
    </div>
    <a class="headerlink" href="#Index.schema"></a>
    
            <div class="docstring"><p>The schema of the current index.</p>
</div>


                            </div>
                            <div id="Index.searcher" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">searcher</span><span class="signature pdoc-code condensed">(<span class="param"><span class="bp">self</span></span><span class="return-annotation">) -> <span class="n"><a href="#Searcher">Searcher</a></span>:</span></span>

        
    </div>
    <a class="headerlink" href="#Index.searcher"></a>
    
            <div class="docstring"><p>Returns a searcher</p>

<p>This method should be called every single time a search query is performed.
The same searcher must be used for a given query, as it ensures the use of a consistent segment set.</p>
</div>


                            </div>
                            <div id="Index.writer" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">writer</span><span class="signature pdoc-code multiline">(<span class="param">    <span class="bp">self</span>,</span><span class="param">    <span class="n">heap_size</span><span class="p">:</span> <span class="nb">int</span> <span class="o">=</span> <span class="mi">128000000</span>,</span><span class="param">    <span class="n">num_threads</span><span class="p">:</span> <span class="nb">int</span> <span class="o">=</span> <span class="mi">0</span></span><span class="return-annotation">) -> <span class="n"><a href="#IndexWriter">IndexWriter</a></span>:</span></span>

        
    </div>
    <a class="headerlink" href="#Index.writer"></a>
    
            <div class="docstring"><p>Create a <code><a href="#IndexWriter">IndexWriter</a></code> for the index.</p>

<p>The writer will be multithreaded and the provided heap size will be
split between the given number of threads.</p>

<h6 id="arguments">Arguments:</h6>

<ul>
<li><strong>overall_heap_size (int, optional):</strong>  The total target heap memory usage of
the writer. Tantivy requires that this can't be less
than 3000000 <em>per thread</em>. Lower values will result in more
frequent internal commits when adding documents (slowing down
write progress), and larger values will results in fewer
commits but greater memory usage. The best value will depend
on your specific use case.</li>
<li><strong>num_threads (int, optional):</strong>  The number of threads that the writer
should use. If this value is 0, tantivy will choose
automatically the number of threads.</li>
</ul>

<p>Raises ValueError if there was an error while creating the writer.</p>
</div>


                            </div>
                </section>
</main>

## IndexWriter

<main class="pdoc">
<section id="IndexWriter">
                    <div class="attr class">
            
    <span class="def">class</span>
    <span class="name">IndexWriter</span>:

        
    </div>
    <a class="headerlink" href="#IndexWriter"></a>
    
            <div class="docstring"><p>IndexWriter is the user entry-point to add documents to the index.</p>

<p>To create an IndexWriter first create an Index and call the writer() method
on the index object.</p>
</div>


                            <div id="IndexWriter.add_document" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">add_document</span><span class="signature pdoc-code condensed">(<span class="param"><span class="bp">self</span>, </span><span class="param"><span class="n">doc</span><span class="p">:</span> <span class="n"><a href="#Document">Document</a></span></span><span class="return-annotation">) -> <span class="nb">int</span>:</span></span>

        
    </div>
    <a class="headerlink" href="#IndexWriter.add_document"></a>
    
            <div class="docstring"><p>Add a document to the index.</p>

<p>If the indexing pipeline is full, this call may block.</p>

<p>Returns an <code>opstamp</code>, which is an increasing integer that can be used
by the client to align commits with its own document queue.
The <code>opstamp</code> represents the number of documents that have been added
since the creation of the index.</p>
</div>


                            </div>
                            <div id="IndexWriter.add_json" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">add_json</span><span class="signature pdoc-code condensed">(<span class="param"><span class="bp">self</span>, </span><span class="param"><span class="n">json</span><span class="p">:</span> <span class="nb">str</span></span><span class="return-annotation">) -> <span class="nb">int</span>:</span></span>

        
    </div>
    <a class="headerlink" href="#IndexWriter.add_json"></a>
    
            <div class="docstring"><p>Helper for the <code><a href="#IndexWriter.add_document">add_document</a></code> method, but passing a json string.</p>

<p>If the indexing pipeline is full, this call may block.</p>

<p>Returns an <code>opstamp</code>, which is an increasing integer that can be used
by the client to align commits with its own document queue.
The <code>opstamp</code> represents the number of documents that have been added
since the creation of the index.</p>
</div>


                            </div>
                            <div id="IndexWriter.commit" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">commit</span><span class="signature pdoc-code condensed">(<span class="param"><span class="bp">self</span></span><span class="return-annotation">) -> <span class="nb">int</span>:</span></span>

        
    </div>
    <a class="headerlink" href="#IndexWriter.commit"></a>
    
            <div class="docstring"><p>Commits all of the pending changes</p>

<p>A call to commit blocks. After it returns, all of the document that
were added since the last commit are published and persisted.</p>

<p>In case of a crash or an hardware failure (as long as the hard disk is
spared), it will be possible to resume indexing from this point.</p>

<p>Returns the <code>opstamp</code> of the last document that made it in the commit.</p>
</div>


                            </div>
                            <div id="IndexWriter.commit_opstamp" class="classattr">
                                <div class="attr variable">
            <span class="name">commit_opstamp</span><span class="annotation">: int</span>

        
    </div>
    <a class="headerlink" href="#IndexWriter.commit_opstamp"></a>
    
            <div class="docstring"><p>The opstamp of the last successful commit.</p>

<p>This is the opstamp the index will rollback to if there is a failure
like a power surge.</p>

<p>This is also the opstamp of the commit that is currently available
for searchers.</p>
</div>


                            </div>
                            <div id="IndexWriter.delete_all_documents" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">delete_all_documents</span><span class="signature pdoc-code condensed">(<span class="param"><span class="bp">self</span></span><span class="return-annotation">) -> <span class="kc">None</span>:</span></span>

        
    </div>
    <a class="headerlink" href="#IndexWriter.delete_all_documents"></a>
    
            <div class="docstring"><p>Deletes all documents from the index.</p>
</div>


                            </div>
                            <div id="IndexWriter.delete_documents" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">delete_documents</span><span class="signature pdoc-code condensed">(<span class="param"><span class="bp">self</span>, </span><span class="param"><span class="n">field_name</span><span class="p">:</span> <span class="nb">str</span>, </span><span class="param"><span class="n">field_value</span><span class="p">:</span> <span class="n">Any</span></span><span class="return-annotation">) -> <span class="nb">int</span>:</span></span>

        
    </div>
    <a class="headerlink" href="#IndexWriter.delete_documents"></a>
    
            <div class="docstring"><p>Deprecated alias of <code><a href="#IndexWriter.delete_documents_by_term">delete_documents_by_term</a></code>; emits a
<code>DeprecationWarning</code>. Use <code><a href="#IndexWriter.delete_documents_by_term">delete_documents_by_term</a></code> or
<code><a href="#IndexWriter.delete_documents_by_query">delete_documents_by_query</a></code> instead.</p>
</div>


                            </div>
                            <div id="IndexWriter.delete_documents_by_query" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">delete_documents_by_query</span><span class="signature pdoc-code condensed">(<span class="param"><span class="bp">self</span>, </span><span class="param"><span class="n">query</span><span class="p">:</span> <span class="n"><a href="#Query">Query</a></span></span><span class="return-annotation">) -> <span class="nb">int</span>:</span></span>

        
    </div>
    <a class="headerlink" href="#IndexWriter.delete_documents_by_query"></a>
    
            <div class="docstring"><p>Delete all documents matching a given query.</p>

<h6 id="example">Example:</h6>

<blockquote>
  <div class="pdoc-code codehilite">
<pre><span></span><code><span class="n">schema_builder</span> <span class="o">=</span> <span class="n">SchemaBuilder</span><span class="p">()</span>
<span class="n">schema_builder</span><span class="o">.</span><span class="n">add_text_field</span><span class="p">(</span><span class="s2">&quot;title&quot;</span><span class="p">,</span> <span class="n">fast</span><span class="o">=</span><span class="kc">True</span><span class="p">)</span>
<span class="n">schema</span> <span class="o">=</span> <span class="n">schema_builder</span><span class="o">.</span><span class="n">build</span><span class="p">()</span>
<span class="n">index</span> <span class="o">=</span> <span class="n">Index</span><span class="p">(</span><span class="n">schema</span><span class="p">)</span>
<span class="n">writer</span> <span class="o">=</span> <span class="n">index</span><span class="o">.</span><span class="n">writer</span><span class="p">()</span>
<span class="n">source_doc</span> <span class="o">=</span> <span class="p">{</span>
    <span class="s2">&quot;title&quot;</span><span class="p">:</span> <span class="s2">&quot;Here is some text&quot;</span>
<span class="p">}</span>
<span class="n">writer</span><span class="o">.</span><span class="n">add_json</span><span class="p">(</span><span class="n">json</span><span class="o">.</span><span class="n">dumps</span><span class="p">(</span><span class="n">source_doc</span><span class="p">))</span>
<span class="n">writer</span><span class="o">.</span><span class="n">commit</span><span class="p">()</span>
<span class="n">writer</span><span class="o">.</span><span class="n">wait_merging_threads</span><span class="p">()</span>

<span class="n">query</span> <span class="o">=</span> <span class="n">index</span><span class="o">.</span><span class="n">parse_query</span><span class="p">(</span><span class="s2">&quot;title:text&quot;</span><span class="p">)</span>
<span class="n">writer</span> <span class="o">=</span> <span class="n">index</span><span class="o">.</span><span class="n">writer</span><span class="p">()</span>
<span class="n">writer</span><span class="o">.</span><span class="n">delete_documents_by_query</span><span class="p">(</span><span class="n">query</span><span class="p">)</span>
<span class="n">writer</span><span class="o">.</span><span class="n">commit</span><span class="p">()</span>
<span class="n">writer</span><span class="o">.</span><span class="n">wait_merging_threads</span><span class="p">()</span>
</code></pre>
  </div>
</blockquote>

<h6 id="arguments">Arguments:</h6>

<ul>
<li><strong>query (Query):</strong>  The query to filter the deleted documents.</li>
</ul>

<p>If the query is not valid raises ValueError exception.
If the query is not supported raises Exception.</p>
</div>


                            </div>
                            <div id="IndexWriter.delete_documents_by_term" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">delete_documents_by_term</span><span class="signature pdoc-code condensed">(<span class="param"><span class="bp">self</span>, </span><span class="param"><span class="n">field_name</span><span class="p">:</span> <span class="nb">str</span>, </span><span class="param"><span class="n">field_value</span><span class="p">:</span> <span class="n">Any</span></span><span class="return-annotation">) -> <span class="nb">int</span>:</span></span>

        
    </div>
    <a class="headerlink" href="#IndexWriter.delete_documents_by_term"></a>
    
            <div class="docstring"><p>Delete all documents containing a given term.</p>

<p>This method does not parse the given term and it expects the term to be
already tokenized according to any tokenizers attached to the field. This
can often result in surprising behaviour. For example, if you want to store
UUIDs as text in a field, and those values have hyphens, and you use the
default tokenizer which removes punctuation, you will not be able to delete
a document added with particular UUID, by passing the same UUID to this
method. In such workflows where deletions are required, particularly with
string values, it is strongly recommended to use the
"raw" tokenizer as this will match exactly. In situations where you do
want tokenization to be applied, it is recommended to instead use the
<code><a href="#IndexWriter.delete_documents_by_query">delete_documents_by_query</a></code> method instead, which will delete documents
matching the given query using the same query parser as used in search queries.</p>

<h6 id="arguments">Arguments:</h6>

<ul>
<li><strong>field_name (str):</strong>  The field name for which we want to filter deleted docs.</li>
<li><strong>field_value (PyAny):</strong>  Python object with the value we want to filter.</li>
</ul>

<p>If the field_name is not on the schema raises ValueError exception.
If the field_value is not supported raises Exception.</p>
</div>


                            </div>
                            <div id="IndexWriter.garbage_collect_files" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">garbage_collect_files</span><span class="signature pdoc-code condensed">(<span class="param"><span class="bp">self</span></span><span class="return-annotation">) -> <span class="kc">None</span>:</span></span>

        
    </div>
    <a class="headerlink" href="#IndexWriter.garbage_collect_files"></a>
    
            <div class="docstring"><p>Detect and removes the files that are not used by the index anymore.</p>
</div>


                            </div>
                            <div id="IndexWriter.rollback" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">rollback</span><span class="signature pdoc-code condensed">(<span class="param"><span class="bp">self</span></span><span class="return-annotation">) -> <span class="nb">int</span>:</span></span>

        
    </div>
    <a class="headerlink" href="#IndexWriter.rollback"></a>
    
            <div class="docstring"><p>Rollback to the last commit</p>

<p>This cancels all of the update that happened before after the last
commit. After calling rollback, the index is in the same state as it
was after the last commit.</p>
</div>


                            </div>
                            <div id="IndexWriter.wait_merging_threads" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">wait_merging_threads</span><span class="signature pdoc-code condensed">(<span class="param"><span class="bp">self</span></span><span class="return-annotation">) -> <span class="kc">None</span>:</span></span>

        
    </div>
    <a class="headerlink" href="#IndexWriter.wait_merging_threads"></a>
    
            <div class="docstring"><p>If there are some merging threads, blocks until they all finish
their work and then drop the <code><a href="#IndexWriter">IndexWriter</a></code>.</p>

<p>This will consume the <code><a href="#IndexWriter">IndexWriter</a></code>. Further accesses to the
object will result in an error.</p>
</div>


                            </div>
                </section>
</main>

## Occur

<main class="pdoc">
<section id="Occur">
                    <div class="attr class">
            
    <span class="def">class</span>
    <span class="name">Occur</span>:

        
    </div>
    <a class="headerlink" href="#Occur"></a>
    
            <div class="docstring"><p>Tantivy's Occur</p>
</div>


                            <div id="Occur.Must" class="classattr">
                                <div class="attr variable">
            <span class="name">Must</span>        =
<span class="default_value"><a href="#Occur.Must">Occur.Must</a></span>

        
    </div>
    <a class="headerlink" href="#Occur.Must"></a>
    
    

                            </div>
                            <div id="Occur.MustNot" class="classattr">
                                <div class="attr variable">
            <span class="name">MustNot</span>        =
<span class="default_value"><a href="#Occur.MustNot">Occur.MustNot</a></span>

        
    </div>
    <a class="headerlink" href="#Occur.MustNot"></a>
    
    

                            </div>
                            <div id="Occur.Should" class="classattr">
                                <div class="attr variable">
            <span class="name">Should</span>        =
<span class="default_value"><a href="#Occur.Should">Occur.Should</a></span>

        
    </div>
    <a class="headerlink" href="#Occur.Should"></a>
    
    

                            </div>
                </section>
</main>

## Order

<main class="pdoc">
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
</main>

## parse_query

<main class="pdoc">
<section id="parse_query">
                    <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">parse_query</span><span class="signature pdoc-code condensed">(<span class="param"><span class="n">query</span><span class="p">:</span> <span class="nb">str</span></span><span class="return-annotation">) -> <span class="nb">dict</span><span class="p">[</span><span class="nb">str</span><span class="p">,</span> <span class="n">typing</span><span class="o">.</span><span class="n">Any</span><span class="p">]</span>:</span></span>

        
    </div>
    <a class="headerlink" href="#parse_query"></a>
    
            <div class="docstring"><p>Parse a query string into an abstract syntax tree (AST).</p>

<h6 id="arguments">Arguments:</h6>

<ul>
<li><strong>query:</strong>  The query string to parse.</li>
</ul>

<h6 id="returns">Returns:</h6>

<blockquote>
  <p>A dictionary representing the parsed query AST.</p>
</blockquote>

<h6 id="raises">Raises:</h6>

<ul>
<li><strong>ValueError:</strong>  If the query has invalid syntax.</li>
</ul>
</div>


                </section>
</main>

## parse_query_lenient

<main class="pdoc">
<section id="parse_query_lenient">
                    <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">parse_query_lenient</span><span class="signature pdoc-code condensed">(<span class="param"><span class="n">query</span><span class="p">:</span> <span class="nb">str</span></span><span class="return-annotation">) -> <span class="nb">tuple</span><span class="p">[</span><span class="nb">dict</span><span class="p">[</span><span class="nb">str</span><span class="p">,</span> <span class="n">typing</span><span class="o">.</span><span class="n">Any</span><span class="p">],</span> <span class="nb">list</span><span class="p">[</span><span class="nb">dict</span><span class="p">[</span><span class="nb">str</span><span class="p">,</span> <span class="n">typing</span><span class="o">.</span><span class="n">Any</span><span class="p">]]]</span>:</span></span>

        
    </div>
    <a class="headerlink" href="#parse_query_lenient"></a>
    
            <div class="docstring"><p>Parse a query string leniently, recovering from syntax errors.</p>

<h6 id="arguments">Arguments:</h6>

<ul>
<li><strong>query:</strong>  The query string to parse.</li>
</ul>

<h6 id="returns">Returns:</h6>

<blockquote>
  <p>A tuple containing:
      - A dictionary representing the parsed query AST
      - A list of error dictionaries describing syntax errors</p>
</blockquote>
</div>


                </section>
</main>

## Query

<main class="pdoc">
<section id="Query">
                    <div class="attr class">
            
    <span class="def">class</span>
    <span class="name">Query</span>:

        
    </div>
    <a class="headerlink" href="#Query"></a>
    
            <div class="docstring"><p>Tantivy's Query</p>
</div>


                            <div id="Query.all_query" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">all_query</span><span class="signature pdoc-code condensed">(<span class="return-annotation">) -> <span class="n"><a href="#Query">Query</a></span>:</span></span>

        
    </div>
    <a class="headerlink" href="#Query.all_query"></a>
    
            <div class="docstring"><p>Construct a Tantivy's AllQuery</p>
</div>


                            </div>
                            <div id="Query.and_must_match" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">and_must_match</span><span class="signature pdoc-code condensed">(<span class="param"><span class="bp">self</span>, </span><span class="param"><span class="o">*</span><span class="n">queries</span><span class="p">:</span> <span class="n"><a href="#Query">Query</a></span></span><span class="return-annotation">) -> <span class="n"><a href="#Query">Query</a></span>:</span></span>

        
    </div>
    <a class="headerlink" href="#Query.and_must_match"></a>
    
            <div class="docstring"><p>Convenience method to combine queries with AND (MUST) logic.
Returns a query matching documents that match this query and every
given query. Accepts any number of queries, so a list can be passed
with argument unpacking: <code>query.and_must_match(*queries)</code>.</p>
</div>


                            </div>
                            <div id="Query.and_must_not_match" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">and_must_not_match</span><span class="signature pdoc-code condensed">(<span class="param"><span class="bp">self</span>, </span><span class="param"><span class="o">*</span><span class="n">queries</span><span class="p">:</span> <span class="n"><a href="#Query">Query</a></span></span><span class="return-annotation">) -> <span class="n"><a href="#Query">Query</a></span>:</span></span>

        
    </div>
    <a class="headerlink" href="#Query.and_must_not_match"></a>
    
            <div class="docstring"><p>Convenience method to combine queries with AND NOT (MUST NOT) logic.
Returns a query matching documents that match this query and none of
the given queries. Accepts any number of queries, so a list can be
passed with argument unpacking: <code>query.and_must_not_match(*queries)</code>.</p>
</div>


                            </div>
                            <div id="Query.boolean_query" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">boolean_query</span><span class="signature pdoc-code multiline">(<span class="param">    <span class="n">subqueries</span><span class="p">:</span> <span class="n">Sequence</span><span class="p">[</span><span class="nb">tuple</span><span class="p">[</span><span class="n"><a href="#Occur">Occur</a></span><span class="p">,</span> <span class="n"><a href="#Query">Query</a></span><span class="p">]]</span>,</span><span class="param">    <span class="n">minimum_number_should_match</span><span class="p">:</span> <span class="nb">int</span> <span class="o">|</span> <span class="kc">None</span> <span class="o">=</span> <span class="kc">None</span></span><span class="return-annotation">) -> <span class="n"><a href="#Query">Query</a></span>:</span></span>

        
    </div>
    <a class="headerlink" href="#Query.boolean_query"></a>
    
            <div class="docstring"><p>Construct a Tantivy's BooleanQuery</p>
</div>


                            </div>
                            <div id="Query.boost_query" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">boost_query</span><span class="signature pdoc-code condensed">(<span class="param"><span class="n">query</span><span class="p">:</span> <span class="n"><a href="#Query">Query</a></span>, </span><span class="param"><span class="n">boost</span><span class="p">:</span> <span class="nb">float</span></span><span class="return-annotation">) -> <span class="n"><a href="#Query">Query</a></span>:</span></span>

        
    </div>
    <a class="headerlink" href="#Query.boost_query"></a>
    
            <div class="docstring"><p>Construct a Tantivy's BoostQuery</p>
</div>


                            </div>
                            <div id="Query.const_score_query" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">const_score_query</span><span class="signature pdoc-code condensed">(<span class="param"><span class="n">query</span><span class="p">:</span> <span class="n"><a href="#Query">Query</a></span>, </span><span class="param"><span class="n">score</span><span class="p">:</span> <span class="nb">float</span></span><span class="return-annotation">) -> <span class="n"><a href="#Query">Query</a></span>:</span></span>

        
    </div>
    <a class="headerlink" href="#Query.const_score_query"></a>
    
            <div class="docstring"><p>Construct a Tantivy's ConstScoreQuery</p>
</div>


                            </div>
                            <div id="Query.disjunction_max_query" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">disjunction_max_query</span><span class="signature pdoc-code multiline">(<span class="param">    <span class="n">subqueries</span><span class="p">:</span> <span class="n">Sequence</span><span class="p">[</span><span class="n"><a href="#Query">Query</a></span><span class="p">]</span>,</span><span class="param">    <span class="n">tie_breaker</span><span class="p">:</span> <span class="nb">float</span> <span class="o">|</span> <span class="kc">None</span> <span class="o">=</span> <span class="kc">None</span></span><span class="return-annotation">) -> <span class="n"><a href="#Query">Query</a></span>:</span></span>

        
    </div>
    <a class="headerlink" href="#Query.disjunction_max_query"></a>
    
            <div class="docstring"><p>Construct a Tantivy's DisjunctionMaxQuery</p>
</div>


                            </div>
                            <div id="Query.empty_query" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">empty_query</span><span class="signature pdoc-code condensed">(<span class="return-annotation">) -> <span class="n"><a href="#Query">Query</a></span>:</span></span>

        
    </div>
    <a class="headerlink" href="#Query.empty_query"></a>
    
            <div class="docstring"><p>Construct a Tantivy's EmptyQuery</p>
</div>


                            </div>
                            <div id="Query.exists_query" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">exists_query</span><span class="signature pdoc-code multiline">(<span class="param">    <span class="n">fast_field_name</span><span class="p">:</span> <span class="nb">str</span>,</span><span class="param">    <span class="n">json_subpaths</span><span class="p">:</span> <span class="nb">bool</span> <span class="o">=</span> <span class="kc">False</span></span><span class="return-annotation">) -> <span class="n"><a href="#Query">Query</a></span>:</span></span>

        
    </div>
    <a class="headerlink" href="#Query.exists_query"></a>
    
            <div class="docstring"><p>Construct a Tantivy's ExistsQuery
Executing a search with this query will fail if the specified field doesn’t exists or is not a fast field.</p>

<h1 id="arguments">Arguments</h1>

<ul>
<li><code>fast_field_name</code> - Field name to be searched.</li>
<li><code>json_subpaths</code> - If true, check all the subpaths inside a JSON field</li>
</ul>
</div>


                            </div>
                            <div id="Query.explain" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">explain</span><span class="signature pdoc-code multiline">(<span class="param">    <span class="bp">self</span>,</span><span class="param">    <span class="n">searcher</span><span class="p">:</span> <span class="n"><a href="#Searcher">Searcher</a></span>,</span><span class="param">    <span class="n">doc_address</span><span class="p">:</span> <span class="n"><a href="#DocAddress">DocAddress</a></span></span><span class="return-annotation">) -> <span class="n"><a href="#Explanation">Explanation</a></span>:</span></span>

        
    </div>
    <a class="headerlink" href="#Query.explain"></a>
    
            <div class="docstring"><p>Explain how this query matches a given document.</p>

<h1 id="arguments">Arguments</h1>

<ul>
<li><code>searcher</code> (Searcher): The searcher used to perform the search.</li>
<li><code>doc_address</code> (DocAddress): The address of the document to explain.</li>
</ul>

<h1 id="returns">Returns</h1>

<ul>
<li><code><a href="#Explanation">Explanation</a></code>: An object containing detailed information about how
the document matched the query, with a to_json() method.</li>
</ul>
</div>


                            </div>
                            <div id="Query.fuzzy_term_query" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">fuzzy_term_query</span><span class="signature pdoc-code multiline">(<span class="param">    <span class="n">schema</span><span class="p">:</span> <span class="n"><a href="#Schema">Schema</a></span>,</span><span class="param">    <span class="n">field_name</span><span class="p">:</span> <span class="nb">str</span>,</span><span class="param">    <span class="n">text</span><span class="p">:</span> <span class="nb">str</span>,</span><span class="param">    <span class="n">distance</span><span class="p">:</span> <span class="nb">int</span> <span class="o">=</span> <span class="mi">1</span>,</span><span class="param">    <span class="n">transposition_cost_one</span><span class="p">:</span> <span class="nb">bool</span> <span class="o">=</span> <span class="kc">True</span>,</span><span class="param">    <span class="n">prefix</span><span class="o">=</span><span class="kc">False</span></span><span class="return-annotation">) -> <span class="n"><a href="#Query">Query</a></span>:</span></span>

        
    </div>
    <a class="headerlink" href="#Query.fuzzy_term_query"></a>
    
            <div class="docstring"><p>Construct a Tantivy's FuzzyTermQuery</p>

<h1 id="arguments">Arguments</h1>

<ul>
<li><code>schema</code> - Schema of the target index.</li>
<li><code>field_name</code> - Field name to be searched.</li>
<li><code>text</code> - String representation of the query term.</li>
<li><code>distance</code> - (Optional) Edit distance you are going to allow. When not specified, the default is 1.</li>
<li><code>transposition_cost_one</code> - (Optional) If true, a transposition (swapping) cost will be 1; otherwise it will be 2. When not specified, the default is true.</li>
<li><code>prefix</code> - (Optional) If true, prefix levenshtein distance is applied. When not specified, the default is false.</li>
</ul>
</div>


                            </div>
                            <div id="Query.more_like_this_document_fields_query" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">more_like_this_document_fields_query</span><span class="signature pdoc-code multiline">(<span class="param">    <span class="n">schema</span><span class="p">:</span> <span class="n"><a href="#Schema">Schema</a></span>,</span><span class="param">    <span class="n">document_fields</span><span class="p">:</span> <span class="nb">dict</span><span class="p">[</span><span class="nb">str</span><span class="p">,</span> <span class="n">typing</span><span class="o">.</span><span class="n">Any</span> <span class="o">|</span> <span class="nb">list</span><span class="p">[</span><span class="n">typing</span><span class="o">.</span><span class="n">Any</span><span class="p">]]</span>,</span><span class="param">    <span class="n">min_doc_frequency</span><span class="p">:</span> <span class="nb">int</span> <span class="o">|</span> <span class="kc">None</span> <span class="o">=</span> <span class="mi">5</span>,</span><span class="param">    <span class="n">max_doc_frequency</span><span class="p">:</span> <span class="nb">int</span> <span class="o">|</span> <span class="kc">None</span> <span class="o">=</span> <span class="kc">None</span>,</span><span class="param">    <span class="n">min_term_frequency</span><span class="p">:</span> <span class="nb">int</span> <span class="o">|</span> <span class="kc">None</span> <span class="o">=</span> <span class="mi">2</span>,</span><span class="param">    <span class="n">max_query_terms</span><span class="p">:</span> <span class="nb">int</span> <span class="o">|</span> <span class="kc">None</span> <span class="o">=</span> <span class="mi">25</span>,</span><span class="param">    <span class="n">min_word_length</span><span class="p">:</span> <span class="nb">int</span> <span class="o">|</span> <span class="kc">None</span> <span class="o">=</span> <span class="kc">None</span>,</span><span class="param">    <span class="n">max_word_length</span><span class="p">:</span> <span class="nb">int</span> <span class="o">|</span> <span class="kc">None</span> <span class="o">=</span> <span class="kc">None</span>,</span><span class="param">    <span class="n">boost_factor</span><span class="p">:</span> <span class="nb">float</span> <span class="o">|</span> <span class="kc">None</span> <span class="o">=</span> <span class="mf">1.0</span>,</span><span class="param">    <span class="n">stop_words</span><span class="p">:</span> <span class="nb">list</span><span class="p">[</span><span class="nb">str</span><span class="p">]</span> <span class="o">=</span> <span class="p">[]</span></span><span class="return-annotation">) -> <span class="n"><a href="#Query">Query</a></span>:</span></span>

        
    </div>
    <a class="headerlink" href="#Query.more_like_this_document_fields_query"></a>
    
            <div class="docstring"><p>Construct a Tantivy's MoreLikeThisQuery from caller-provided field values.</p>
</div>


                            </div>
                            <div id="Query.more_like_this_query" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">more_like_this_query</span><span class="signature pdoc-code multiline">(<span class="param">    <span class="n">doc_address</span><span class="p">:</span> <span class="n"><a href="#DocAddress">DocAddress</a></span>,</span><span class="param">    <span class="n">min_doc_frequency</span><span class="p">:</span> <span class="nb">int</span> <span class="o">|</span> <span class="kc">None</span> <span class="o">=</span> <span class="mi">5</span>,</span><span class="param">    <span class="n">max_doc_frequency</span><span class="p">:</span> <span class="nb">int</span> <span class="o">|</span> <span class="kc">None</span> <span class="o">=</span> <span class="kc">None</span>,</span><span class="param">    <span class="n">min_term_frequency</span><span class="p">:</span> <span class="nb">int</span> <span class="o">|</span> <span class="kc">None</span> <span class="o">=</span> <span class="mi">2</span>,</span><span class="param">    <span class="n">max_query_terms</span><span class="p">:</span> <span class="nb">int</span> <span class="o">|</span> <span class="kc">None</span> <span class="o">=</span> <span class="mi">25</span>,</span><span class="param">    <span class="n">min_word_length</span><span class="p">:</span> <span class="nb">int</span> <span class="o">|</span> <span class="kc">None</span> <span class="o">=</span> <span class="kc">None</span>,</span><span class="param">    <span class="n">max_word_length</span><span class="p">:</span> <span class="nb">int</span> <span class="o">|</span> <span class="kc">None</span> <span class="o">=</span> <span class="kc">None</span>,</span><span class="param">    <span class="n">boost_factor</span><span class="p">:</span> <span class="nb">float</span> <span class="o">|</span> <span class="kc">None</span> <span class="o">=</span> <span class="mf">1.0</span>,</span><span class="param">    <span class="n">stop_words</span><span class="p">:</span> <span class="nb">list</span><span class="p">[</span><span class="nb">str</span><span class="p">]</span> <span class="o">=</span> <span class="p">[]</span></span><span class="return-annotation">) -> <span class="n"><a href="#Query">Query</a></span>:</span></span>

        
    </div>
    <a class="headerlink" href="#Query.more_like_this_query"></a>
    
            <div class="docstring"><p>The type of the None singleton.</p>
</div>


                            </div>
                            <div id="Query.or_should_match" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">or_should_match</span><span class="signature pdoc-code condensed">(<span class="param"><span class="bp">self</span>, </span><span class="param"><span class="o">*</span><span class="n">queries</span><span class="p">:</span> <span class="n"><a href="#Query">Query</a></span></span><span class="return-annotation">) -> <span class="n"><a href="#Query">Query</a></span>:</span></span>

        
    </div>
    <a class="headerlink" href="#Query.or_should_match"></a>
    
            <div class="docstring"><p>Convenience method to combine queries with OR (SHOULD) logic.
Returns a query matching documents that match this query or any of
the given queries. Accepts any number of queries, so a list can be
passed with argument unpacking: <code>query.or_should_match(*queries)</code>.</p>
</div>


                            </div>
                            <div id="Query.phrase_prefix_query" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">phrase_prefix_query</span><span class="signature pdoc-code multiline">(<span class="param">    <span class="n">schema</span><span class="p">:</span> <span class="n"><a href="#Schema">Schema</a></span>,</span><span class="param">    <span class="n">field_name</span><span class="p">:</span> <span class="nb">str</span>,</span><span class="param">    <span class="n">words</span><span class="p">:</span> <span class="nb">list</span><span class="p">[</span><span class="nb">str</span> <span class="o">|</span> <span class="nb">tuple</span><span class="p">[</span><span class="nb">int</span><span class="p">,</span> <span class="nb">str</span><span class="p">]]</span></span><span class="return-annotation">) -> <span class="n"><a href="#Query">Query</a></span>:</span></span>

        
    </div>
    <a class="headerlink" href="#Query.phrase_prefix_query"></a>
    
            <div class="docstring"><p>Construct a Tantivy's PhrasePrefixQuery with custom offsets and slop</p>

<h1 id="arguments">Arguments</h1>

<ul>
<li><code>schema</code> - Schema of the target index.</li>
<li><code>field_name</code> - Field name to be searched.</li>
<li><code>words</code> - Word list that constructs the phrase. A word can be a term text or a pair of term text and its offset in the phrase.</li>
</ul>
</div>


                            </div>
                            <div id="Query.phrase_query" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">phrase_query</span><span class="signature pdoc-code multiline">(<span class="param">    <span class="n">schema</span><span class="p">:</span> <span class="n"><a href="#Schema">Schema</a></span>,</span><span class="param">    <span class="n">field_name</span><span class="p">:</span> <span class="nb">str</span>,</span><span class="param">    <span class="n">words</span><span class="p">:</span> <span class="nb">list</span><span class="p">[</span><span class="nb">str</span> <span class="o">|</span> <span class="nb">tuple</span><span class="p">[</span><span class="nb">int</span><span class="p">,</span> <span class="nb">str</span><span class="p">]]</span>,</span><span class="param">    <span class="n">slop</span><span class="p">:</span> <span class="nb">int</span> <span class="o">=</span> <span class="mi">0</span></span><span class="return-annotation">) -> <span class="n"><a href="#Query">Query</a></span>:</span></span>

        
    </div>
    <a class="headerlink" href="#Query.phrase_query"></a>
    
            <div class="docstring"><p>Construct a Tantivy's PhraseQuery with custom offsets and slop</p>

<h1 id="arguments">Arguments</h1>

<ul>
<li><code>schema</code> - Schema of the target index.</li>
<li><code>field_name</code> - Field name to be searched.</li>
<li><code>words</code> - Word list that constructs the phrase. A word can be a term text or a pair of term text and its offset in the phrase.</li>
<li><code>slop</code> - (Optional) The number of gaps permitted between the words in the query phrase. Default is 0.</li>
</ul>
</div>


                            </div>
                            <div id="Query.range_query" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">range_query</span><span class="signature pdoc-code multiline">(<span class="param">    <span class="n">schema</span><span class="p">:</span> <span class="n"><a href="#Schema">Schema</a></span>,</span><span class="param">    <span class="n">field_name</span><span class="p">:</span> <span class="nb">str</span>,</span><span class="param">    <span class="n">field_type</span><span class="p">:</span> <span class="n"><a href="#FieldType">FieldType</a></span>,</span><span class="param">    <span class="n">lower_bound</span><span class="p">:</span> <span class="o">~</span><span class="n">_RangeType</span> <span class="o">|</span> <span class="kc">None</span> <span class="o">=</span> <span class="kc">None</span>,</span><span class="param">    <span class="n">upper_bound</span><span class="p">:</span> <span class="o">~</span><span class="n">_RangeType</span> <span class="o">|</span> <span class="kc">None</span> <span class="o">=</span> <span class="kc">None</span>,</span><span class="param">    <span class="n">include_lower</span><span class="p">:</span> <span class="nb">bool</span> <span class="o">=</span> <span class="kc">True</span>,</span><span class="param">    <span class="n">include_upper</span><span class="p">:</span> <span class="nb">bool</span> <span class="o">=</span> <span class="kc">True</span>,</span><span class="param">    <span class="n">use_inverted_index</span><span class="p">:</span> <span class="nb">bool</span> <span class="o">=</span> <span class="kc">False</span></span><span class="return-annotation">) -> <span class="n"><a href="#Query">Query</a></span>:</span></span>

        
    </div>
    <a class="headerlink" href="#Query.range_query"></a>
    
            <div class="docstring"><p>Construct a range query over a numeric, date, or IP address field.</p>

<p>Pass <code>None</code> for <code>lower_bound</code> or <code>upper_bound</code> to leave that side unbounded.
Both bounds cannot be <code>None</code>; use <code><a href="#Query.all_query">Query.all_query()</a></code> to match all documents.
Setting <code>include_lower</code> or <code>include_upper</code> to <code>False</code> while the corresponding
bound is <code>None</code> is an error—unbounded sides are always inclusive by definition.</p>

<h1 id="arguments">Arguments</h1>

<ul>
<li><code>schema</code> - Schema of the target index.</li>
<li><code>field_name</code> - Field name to be searched.</li>
<li><code>field_type</code> - Type of the field (<code><a href="#FieldType.Integer">FieldType.Integer</a></code>, <code><a href="#FieldType.Float">FieldType.Float</a></code>, <code><a href="#FieldType.Date">FieldType.Date</a></code>, etc.).</li>
<li><code>lower_bound</code> - Lower bound value, or <code>None</code> for unbounded.</li>
<li><code>upper_bound</code> - Upper bound value, or <code>None</code> for unbounded.</li>
<li><code>include_lower</code> - Whether the lower bound is inclusive. Ignored (and must be <code>True</code>) when <code>lower_bound</code> is <code>None</code>.</li>
<li><code>include_upper</code> - Whether the upper bound is inclusive. Ignored (and must be <code>True</code>) when <code>upper_bound</code> is <code>None</code>.</li>
<li><code>use_inverted_index</code> - If <code>True</code>, use an inverted index range query instead of a fast-field range query.</li>
</ul>
</div>


                            </div>
                            <div id="Query.regex_phrase_query" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">regex_phrase_query</span><span class="signature pdoc-code multiline">(<span class="param">    <span class="n">schema</span><span class="p">:</span> <span class="n"><a href="#Schema">Schema</a></span>,</span><span class="param">    <span class="n">field_name</span><span class="p">:</span> <span class="nb">str</span>,</span><span class="param">    <span class="n">words</span><span class="p">:</span> <span class="nb">list</span><span class="p">[</span><span class="nb">str</span> <span class="o">|</span> <span class="nb">tuple</span><span class="p">[</span><span class="nb">int</span><span class="p">,</span> <span class="nb">str</span><span class="p">]]</span>,</span><span class="param">    <span class="n">slop</span><span class="p">:</span> <span class="nb">int</span> <span class="o">=</span> <span class="mi">0</span></span><span class="return-annotation">) -> <span class="n"><a href="#Query">Query</a></span>:</span></span>

        
    </div>
    <a class="headerlink" href="#Query.regex_phrase_query"></a>
    
            <div class="docstring"><p>Construct a Tantivy's PhraseQuery with custom offsets and slop</p>

<h1 id="arguments">Arguments</h1>

<ul>
<li><code>schema</code> - Schema of the target index.</li>
<li><code>field_name</code> - Field name to be searched.</li>
<li><code>words</code> - Word list that constructs the phrase. A word can be a term text or a pair of term text and its offset in the phrase.</li>
<li><code>slop</code> - (Optional) The number of gaps permitted between the words in the query phrase. Default is 0.</li>
</ul>
</div>


                            </div>
                            <div id="Query.regex_query" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">regex_query</span><span class="signature pdoc-code multiline">(<span class="param">    <span class="n">schema</span><span class="p">:</span> <span class="n"><a href="#Schema">Schema</a></span>,</span><span class="param">    <span class="n">field_name</span><span class="p">:</span> <span class="nb">str</span>,</span><span class="param">    <span class="n">regex_pattern</span><span class="p">:</span> <span class="nb">str</span></span><span class="return-annotation">) -> <span class="n"><a href="#Query">Query</a></span>:</span></span>

        
    </div>
    <a class="headerlink" href="#Query.regex_query"></a>
    
            <div class="docstring"><p>Construct a Tantivy's RegexQuery</p>
</div>


                            </div>
                            <div id="Query.term_query" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">term_query</span><span class="signature pdoc-code multiline">(<span class="param">    <span class="n">schema</span><span class="p">:</span> <span class="n"><a href="#Schema">Schema</a></span>,</span><span class="param">    <span class="n">field_name</span><span class="p">:</span> <span class="nb">str</span>,</span><span class="param">    <span class="n">field_value</span><span class="p">:</span> <span class="n">Any</span>,</span><span class="param">    <span class="n">index_option</span><span class="p">:</span> <span class="nb">str</span> <span class="o">=</span> <span class="s1">&#39;position&#39;</span></span><span class="return-annotation">) -> <span class="n"><a href="#Query">Query</a></span>:</span></span>

        
    </div>
    <a class="headerlink" href="#Query.term_query"></a>
    
            <div class="docstring"><p>Construct a Tantivy's TermQuery</p>
</div>


                            </div>
                            <div id="Query.term_set_query" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">term_set_query</span><span class="signature pdoc-code multiline">(<span class="param">    <span class="n">schema</span><span class="p">:</span> <span class="n"><a href="#Schema">Schema</a></span>,</span><span class="param">    <span class="n">field_name</span><span class="p">:</span> <span class="nb">str</span>,</span><span class="param">    <span class="n">field_values</span><span class="p">:</span> <span class="n">Sequence</span><span class="p">[</span><span class="n">Any</span><span class="p">]</span></span><span class="return-annotation">) -> <span class="n"><a href="#Query">Query</a></span>:</span></span>

        
    </div>
    <a class="headerlink" href="#Query.term_set_query"></a>
    
            <div class="docstring"><p>Construct a Tantivy's TermSetQuery</p>
</div>


                            </div>
                </section>
</main>

## Schema

<main class="pdoc">
<section id="Schema">
                    <div class="attr class">
            
    <span class="def">class</span>
    <span class="name">Schema</span>:

        
    </div>
    <a class="headerlink" href="#Schema"></a>
    
            <div class="docstring"><p>Tantivy schema.</p>

<p>The schema is very strict. To build the schema the <code><a href="#SchemaBuilder">SchemaBuilder</a></code> class is
provided.</p>
</div>


                </section>
</main>

## SchemaBuilder

<main class="pdoc">
<section id="SchemaBuilder">
                    <div class="attr class">
            
    <span class="def">class</span>
    <span class="name">SchemaBuilder</span>:

        
    </div>
    <a class="headerlink" href="#SchemaBuilder"></a>
    
            <div class="docstring"><p>Tantivy has a very strict schema.
You need to specify in advance whether a field is indexed or not,
stored or not.</p>

<p>This is done by creating a schema object, and
setting up the fields one by one.</p>

<h6 id="examples">Examples:</h6>

<blockquote>
  <div class="pdoc-code codehilite">
<pre><span></span><code><span class="gp">&gt;&gt;&gt; </span><span class="n">builder</span> <span class="o">=</span> <span class="n"><a href="#SchemaBuilder">tantivy.SchemaBuilder</a></span><span class="p">()</span>
</code></pre>
  </div>
  
  <div class="pdoc-code codehilite">
<pre><span></span><code><span class="gp">&gt;&gt;&gt; </span><span class="n">title</span> <span class="o">=</span> <span class="n">builder</span><span class="o">.</span><span class="n">add_text_field</span><span class="p">(</span><span class="s2">&quot;title&quot;</span><span class="p">,</span> <span class="n">stored</span><span class="o">=</span><span class="kc">True</span><span class="p">)</span>
<span class="gp">&gt;&gt;&gt; </span><span class="n">body</span> <span class="o">=</span> <span class="n">builder</span><span class="o">.</span><span class="n">add_text_field</span><span class="p">(</span><span class="s2">&quot;body&quot;</span><span class="p">)</span>
</code></pre>
  </div>
  
  <div class="pdoc-code codehilite">
<pre><span></span><code><span class="gp">&gt;&gt;&gt; </span><span class="n">schema</span> <span class="o">=</span> <span class="n">builder</span><span class="o">.</span><span class="n">build</span><span class="p">()</span>
</code></pre>
  </div>
</blockquote>
</div>


                            <div id="SchemaBuilder.add_boolean_field" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">add_boolean_field</span><span class="signature pdoc-code multiline">(<span class="param">    <span class="bp">self</span>,</span><span class="param">    <span class="n">name</span><span class="p">:</span> <span class="nb">str</span>,</span><span class="param">    <span class="n">stored</span><span class="p">:</span> <span class="nb">bool</span> <span class="o">=</span> <span class="kc">False</span>,</span><span class="param">    <span class="n">indexed</span><span class="p">:</span> <span class="nb">bool</span> <span class="o">=</span> <span class="kc">False</span>,</span><span class="param">    <span class="n">fast</span><span class="p">:</span> <span class="nb">bool</span> <span class="o">=</span> <span class="kc">False</span></span><span class="return-annotation">) -> <span class="n"><a href="#SchemaBuilder">SchemaBuilder</a></span>:</span></span>

        
    </div>
    <a class="headerlink" href="#SchemaBuilder.add_boolean_field"></a>
    
            <div class="docstring"><p>Add a new boolean field to the schema.</p>

<h6 id="arguments">Arguments:</h6>

<ul>
<li><strong>name (str):</strong>  The name of the field.</li>
<li><strong>stored (bool, optional):</strong>  If true sets the field as stored, the
content of the field can be later restored from a Searcher.
Defaults to False.</li>
<li><strong>indexed (bool, optional):</strong>  If true sets the field to be indexed.</li>
<li><strong>fast (bool, optional):</strong>  Set the numeric options as a fast field. A
fast field is a column-oriented fashion storage for tantivy.
It is designed for the fast random access of some document
fields given a document id.</li>
</ul>

<p>Returns the associated field handle.
Raises a ValueError if there was an error with the field creation.</p>
</div>


                            </div>
                            <div id="SchemaBuilder.add_bytes_field" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">add_bytes_field</span><span class="signature pdoc-code multiline">(<span class="param">    <span class="bp">self</span>,</span><span class="param">    <span class="n">name</span><span class="p">:</span> <span class="nb">str</span>,</span><span class="param">    <span class="n">stored</span><span class="p">:</span> <span class="nb">bool</span> <span class="o">=</span> <span class="kc">False</span>,</span><span class="param">    <span class="n">indexed</span><span class="p">:</span> <span class="nb">bool</span> <span class="o">=</span> <span class="kc">False</span>,</span><span class="param">    <span class="n">fast</span><span class="p">:</span> <span class="nb">bool</span> <span class="o">=</span> <span class="kc">False</span>,</span><span class="param">    <span class="n">index_option</span><span class="p">:</span> <span class="nb">str</span> <span class="o">=</span> <span class="s1">&#39;position&#39;</span></span><span class="return-annotation">) -> <span class="n"><a href="#SchemaBuilder">SchemaBuilder</a></span>:</span></span>

        
    </div>
    <a class="headerlink" href="#SchemaBuilder.add_bytes_field"></a>
    
            <div class="docstring"><p>Add a fast bytes field to the schema.</p>

<h6 id="arguments">Arguments:</h6>

<ul>
<li><strong>name (str):</strong>  The name of the field.</li>
<li><strong>stored (bool, optional):</strong>  If true sets the field as stored, the
content of the field can be later restored from a Searcher.
Defaults to False.</li>
<li><strong>indexed (bool, optional):</strong>  If true sets the field to be indexed.</li>
<li><strong>fast (bool, optional):</strong>  Set the bytes options as a fast field. A fast
field is a column-oriented fashion storage for tantivy. It is
designed for the fast random access of some document fields
given a document id.</li>
</ul>
</div>


                            </div>
                            <div id="SchemaBuilder.add_date_field" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">add_date_field</span><span class="signature pdoc-code multiline">(<span class="param">    <span class="bp">self</span>,</span><span class="param">    <span class="n">name</span><span class="p">:</span> <span class="nb">str</span>,</span><span class="param">    <span class="n">stored</span><span class="p">:</span> <span class="nb">bool</span> <span class="o">=</span> <span class="kc">False</span>,</span><span class="param">    <span class="n">indexed</span><span class="p">:</span> <span class="nb">bool</span> <span class="o">=</span> <span class="kc">False</span>,</span><span class="param">    <span class="n">fast</span><span class="p">:</span> <span class="nb">bool</span> <span class="o">=</span> <span class="kc">False</span></span><span class="return-annotation">) -> <span class="n"><a href="#SchemaBuilder">SchemaBuilder</a></span>:</span></span>

        
    </div>
    <a class="headerlink" href="#SchemaBuilder.add_date_field"></a>
    
            <div class="docstring"><p>Add a new date field to the schema.</p>

<h6 id="arguments">Arguments:</h6>

<ul>
<li><strong>name (str):</strong>  The name of the field.</li>
<li><strong>stored (bool, optional):</strong>  If true sets the field as stored, the
content of the field can be later restored from a Searcher.
Defaults to False.</li>
<li><strong>indexed (bool, optional):</strong>  If true sets the field to be indexed.</li>
<li><strong>fast (bool, optional):</strong>  Set the date options as a fast field. A fast
field is a column-oriented fashion storage for tantivy. It is
designed for the fast random access of some document fields
given a document id.</li>
</ul>

<p>Returns the associated field handle.
Raises a ValueError if there was an error with the field creation.</p>
</div>


                            </div>
                            <div id="SchemaBuilder.add_facet_field" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">add_facet_field</span><span class="signature pdoc-code condensed">(<span class="param"><span class="bp">self</span>, </span><span class="param"><span class="n">name</span><span class="p">:</span> <span class="nb">str</span></span><span class="return-annotation">) -> <span class="n"><a href="#SchemaBuilder">SchemaBuilder</a></span>:</span></span>

        
    </div>
    <a class="headerlink" href="#SchemaBuilder.add_facet_field"></a>
    
            <div class="docstring"><p>Add a Facet field to the schema.</p>

<h6 id="arguments">Arguments:</h6>

<ul>
<li><strong>name (str):</strong>  The name of the field.</li>
</ul>
</div>


                            </div>
                            <div id="SchemaBuilder.add_float_field" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">add_float_field</span><span class="signature pdoc-code multiline">(<span class="param">    <span class="bp">self</span>,</span><span class="param">    <span class="n">name</span><span class="p">:</span> <span class="nb">str</span>,</span><span class="param">    <span class="n">stored</span><span class="p">:</span> <span class="nb">bool</span> <span class="o">=</span> <span class="kc">False</span>,</span><span class="param">    <span class="n">indexed</span><span class="p">:</span> <span class="nb">bool</span> <span class="o">=</span> <span class="kc">False</span>,</span><span class="param">    <span class="n">fast</span><span class="p">:</span> <span class="nb">bool</span> <span class="o">=</span> <span class="kc">False</span></span><span class="return-annotation">) -> <span class="n"><a href="#SchemaBuilder">SchemaBuilder</a></span>:</span></span>

        
    </div>
    <a class="headerlink" href="#SchemaBuilder.add_float_field"></a>
    
            <div class="docstring"><p>Add a new float field to the schema.</p>

<h6 id="arguments">Arguments:</h6>

<ul>
<li><strong>name (str):</strong>  The name of the field.</li>
<li><strong>stored (bool, optional):</strong>  If true sets the field as stored, the
content of the field can be later restored from a Searcher.
Defaults to False.</li>
<li><strong>indexed (bool, optional):</strong>  If true sets the field to be indexed.</li>
<li><strong>fast (bool, optional):</strong>  Set the numeric options as a fast field. A
fast field is a column-oriented fashion storage for tantivy.
It is designed for the fast random access of some document
fields given a document id.</li>
</ul>

<p>Returns the associated field handle.
Raises a ValueError if there was an error with the field creation.</p>
</div>


                            </div>
                            <div id="SchemaBuilder.add_integer_field" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">add_integer_field</span><span class="signature pdoc-code multiline">(<span class="param">    <span class="bp">self</span>,</span><span class="param">    <span class="n">name</span><span class="p">:</span> <span class="nb">str</span>,</span><span class="param">    <span class="n">stored</span><span class="p">:</span> <span class="nb">bool</span> <span class="o">=</span> <span class="kc">False</span>,</span><span class="param">    <span class="n">indexed</span><span class="p">:</span> <span class="nb">bool</span> <span class="o">=</span> <span class="kc">False</span>,</span><span class="param">    <span class="n">fast</span><span class="p">:</span> <span class="nb">bool</span> <span class="o">=</span> <span class="kc">False</span></span><span class="return-annotation">) -> <span class="n"><a href="#SchemaBuilder">SchemaBuilder</a></span>:</span></span>

        
    </div>
    <a class="headerlink" href="#SchemaBuilder.add_integer_field"></a>
    
            <div class="docstring"><p>Add a new signed integer field to the schema.</p>

<h6 id="arguments">Arguments:</h6>

<ul>
<li><strong>name (str):</strong>  The name of the field.</li>
<li><strong>stored (bool, optional):</strong>  If true sets the field as stored, the
content of the field can be later restored from a Searcher.
Defaults to False.</li>
<li><strong>indexed (bool, optional):</strong>  If true sets the field to be indexed.</li>
<li><strong>fast (bool, optional):</strong>  Set the numeric options as a fast field. A
fast field is a column-oriented fashion storage for tantivy.
It is designed for the fast random access of some document
fields given a document id.</li>
</ul>

<p>Returns the associated field handle.
Raises a ValueError if there was an error with the field creation.</p>
</div>


                            </div>
                            <div id="SchemaBuilder.add_ip_addr_field" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">add_ip_addr_field</span><span class="signature pdoc-code multiline">(<span class="param">    <span class="bp">self</span>,</span><span class="param">    <span class="n">name</span><span class="p">:</span> <span class="nb">str</span>,</span><span class="param">    <span class="n">stored</span><span class="p">:</span> <span class="nb">bool</span> <span class="o">=</span> <span class="kc">False</span>,</span><span class="param">    <span class="n">indexed</span><span class="p">:</span> <span class="nb">bool</span> <span class="o">=</span> <span class="kc">False</span>,</span><span class="param">    <span class="n">fast</span><span class="p">:</span> <span class="nb">bool</span> <span class="o">=</span> <span class="kc">False</span></span><span class="return-annotation">) -> <span class="n"><a href="#SchemaBuilder">SchemaBuilder</a></span>:</span></span>

        
    </div>
    <a class="headerlink" href="#SchemaBuilder.add_ip_addr_field"></a>
    
            <div class="docstring"><p>Add an IP address field to the schema.</p>

<h6 id="arguments">Arguments:</h6>

<ul>
<li><strong>name (str):</strong>  The name of the field.</li>
<li><strong>stored (bool, optional):</strong>  If true sets the field as stored, the
content of the field can be later restored from a Searcher.
Defaults to False.</li>
<li><strong>indexed (bool, optional):</strong>  If true sets the field to be indexed.</li>
<li><strong>fast (bool, optional):</strong>  Set the IP address options as a fast field. A
fast field is a column-oriented fashion storage for tantivy. It
is designed for the fast random access of some document fields
given a document id.</li>
</ul>
</div>


                            </div>
                            <div id="SchemaBuilder.add_json_field" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">add_json_field</span><span class="signature pdoc-code multiline">(<span class="param">    <span class="bp">self</span>,</span><span class="param">    <span class="n">name</span><span class="p">:</span> <span class="nb">str</span>,</span><span class="param">    <span class="n">stored</span><span class="p">:</span> <span class="nb">bool</span> <span class="o">=</span> <span class="kc">False</span>,</span><span class="param">    <span class="n">tokenizer_name</span><span class="p">:</span> <span class="nb">str</span> <span class="o">=</span> <span class="s1">&#39;default&#39;</span>,</span><span class="param">    <span class="n">index_option</span><span class="p">:</span> <span class="nb">str</span> <span class="o">=</span> <span class="s1">&#39;position&#39;</span></span><span class="return-annotation">) -> <span class="n"><a href="#SchemaBuilder">SchemaBuilder</a></span>:</span></span>

        
    </div>
    <a class="headerlink" href="#SchemaBuilder.add_json_field"></a>
    
            <div class="docstring"><p>Add a new json field to the schema.</p>

<h6 id="arguments">Arguments:</h6>

<ul>
<li><strong>name (str):</strong>  the name of the field.</li>
<li><strong>stored (bool, optional):</strong>  If true sets the field as stored, the
content of the field can be later restored from a Searcher.
Defaults to False.</li>
<li><strong>fast (bool, optional):</strong>  Set the text options as a fast field. A
fast field is a column-oriented fashion storage for tantivy.
Text fast fields will have the term ids stored in the fast
field. The fast field will be a multivalued fast field.
It is recommended to use the "raw" tokenizer, since it will
store the original text unchanged. The "default" tokenizer will
store the terms as lower case and this will be reflected in the
dictionary.</li>
<li><strong>tokenizer_name (str, optional):</strong>  The name of the tokenizer that
should be used to process the field. Defaults to 'default'</li>
<li><strong>index_option (str, optional):</strong>  Sets which information should be
indexed with the tokens. Can be one of 'position', 'freq' or
'basic'. Defaults to 'position'. The 'basic' index_option
records only the document ID, the 'freq' option records the
document id and the term frequency, while the 'position' option
records the document id, term frequency and the positions of
the term occurrences in the document.</li>
</ul>

<p>Returns the associated field handle.
Raises a ValueError if there was an error with the field creation.</p>
</div>


                            </div>
                            <div id="SchemaBuilder.add_text_field" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">add_text_field</span><span class="signature pdoc-code multiline">(<span class="param">    <span class="bp">self</span>,</span><span class="param">    <span class="n">name</span><span class="p">:</span> <span class="nb">str</span>,</span><span class="param">    <span class="n">stored</span><span class="p">:</span> <span class="nb">bool</span> <span class="o">=</span> <span class="kc">False</span>,</span><span class="param">    <span class="n">fast</span><span class="p">:</span> <span class="nb">bool</span> <span class="o">=</span> <span class="kc">False</span>,</span><span class="param">    <span class="n">tokenizer_name</span><span class="p">:</span> <span class="nb">str</span> <span class="o">=</span> <span class="s1">&#39;default&#39;</span>,</span><span class="param">    <span class="n">index_option</span><span class="p">:</span> <span class="nb">str</span> <span class="o">=</span> <span class="s1">&#39;position&#39;</span></span><span class="return-annotation">) -> <span class="n"><a href="#SchemaBuilder">SchemaBuilder</a></span>:</span></span>

        
    </div>
    <a class="headerlink" href="#SchemaBuilder.add_text_field"></a>
    
            <div class="docstring"><p>Add a new text field to the schema.</p>

<h6 id="arguments">Arguments:</h6>

<ul>
<li><strong>name (str):</strong>  The name of the field.</li>
<li><strong>stored (bool, optional):</strong>  If true sets the field as stored, the
content of the field can be later restored from a Searcher.
Defaults to False.</li>
<li><strong>fast (bool, optional):</strong>  Set the text options as a fast field. A
fast field is a column-oriented fashion storage for tantivy.
Text fast fields will have the term ids stored in the fast
field. The fast field will be a multivalued fast field.
It is recommended to use the "raw" tokenizer, since it will
store the original text unchanged. The "default" tokenizer will
store the terms as lower case and this will be reflected in the
dictionary.</li>
<li><strong>tokenizer_name (str, optional):</strong>  The name of the tokenizer that
should be used to process the field. Defaults to 'default'</li>
<li><strong>index_option (str, optional):</strong>  Sets which information should be
indexed with the tokens. Can be one of 'position', 'freq' or
'basic'. Defaults to 'position'. The 'basic' index_option
records only the document ID, the 'freq' option records the
document id and the term frequency, while the 'position' option
records the document id, term frequency and the positions of
the term occurrences in the document.</li>
</ul>

<p>Returns the associated field handle.
Raises a ValueError if there was an error with the field creation.</p>
</div>


                            </div>
                            <div id="SchemaBuilder.add_unsigned_field" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">add_unsigned_field</span><span class="signature pdoc-code multiline">(<span class="param">    <span class="bp">self</span>,</span><span class="param">    <span class="n">name</span><span class="p">:</span> <span class="nb">str</span>,</span><span class="param">    <span class="n">stored</span><span class="p">:</span> <span class="nb">bool</span> <span class="o">=</span> <span class="kc">False</span>,</span><span class="param">    <span class="n">indexed</span><span class="p">:</span> <span class="nb">bool</span> <span class="o">=</span> <span class="kc">False</span>,</span><span class="param">    <span class="n">fast</span><span class="p">:</span> <span class="nb">bool</span> <span class="o">=</span> <span class="kc">False</span></span><span class="return-annotation">) -> <span class="n"><a href="#SchemaBuilder">SchemaBuilder</a></span>:</span></span>

        
    </div>
    <a class="headerlink" href="#SchemaBuilder.add_unsigned_field"></a>
    
            <div class="docstring"><p>Add a new unsigned integer field to the schema.</p>

<h6 id="arguments">Arguments:</h6>

<ul>
<li><strong>name (str):</strong>  The name of the field.</li>
<li><strong>stored (bool, optional):</strong>  If true sets the field as stored, the
content of the field can be later restored from a Searcher.
Defaults to False.</li>
<li><strong>indexed (bool, optional):</strong>  If true sets the field to be indexed.</li>
<li><strong>fast (bool, optional):</strong>  Set the numeric options as a fast field. A
fast field is a column-oriented fashion storage for tantivy.
It is designed for the fast random access of some document
fields given a document id.</li>
</ul>

<p>Returns the associated field handle.
Raises a ValueError if there was an error with the field creation.</p>
</div>


                            </div>
                            <div id="SchemaBuilder.build" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">build</span><span class="signature pdoc-code condensed">(<span class="param"><span class="bp">self</span></span><span class="return-annotation">) -> <span class="n"><a href="#Schema">Schema</a></span>:</span></span>

        
    </div>
    <a class="headerlink" href="#SchemaBuilder.build"></a>
    
            <div class="docstring"><p>Finalize the creation of a Schema.</p>

<p>Returns a Schema object. After this is called the SchemaBuilder cannot
be used anymore.</p>
</div>


                            </div>
                            <div id="SchemaBuilder.is_valid_field_name" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">is_valid_field_name</span><span class="signature pdoc-code condensed">(<span class="param"><span class="n">name</span><span class="p">:</span> <span class="nb">str</span></span><span class="return-annotation">) -> <span class="nb">bool</span>:</span></span>

        
    </div>
    <a class="headerlink" href="#SchemaBuilder.is_valid_field_name"></a>
    
            <div class="docstring"><p>The type of the None singleton.</p>
</div>


                            </div>
                </section>
</main>

## Searcher

<main class="pdoc">
<section id="Searcher">
                    <div class="attr class">
            
    <span class="def">class</span>
    <span class="name">Searcher</span>:

        
    </div>
    <a class="headerlink" href="#Searcher"></a>
    
            <div class="docstring"><p>Tantivy's Searcher class</p>

<p>A Searcher is used to search the index given a prepared Query.</p>
</div>


                            <div id="Searcher.aggregate" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">aggregate</span><span class="signature pdoc-code condensed">(<span class="param"><span class="bp">self</span>, </span><span class="param"><span class="n">query</span><span class="p">:</span> <span class="n"><a href="#Query">Query</a></span>, </span><span class="param"><span class="n">agg</span><span class="p">:</span> <span class="nb">dict</span></span><span class="return-annotation">) -> <span class="nb">dict</span>:</span></span>

        
    </div>
    <a class="headerlink" href="#Searcher.aggregate"></a>
    
            <div class="docstring"><p>Execute an aggregation query and return the results as a dict.</p>

<h6 id="arguments">Arguments:</h6>

<ul>
<li><strong>query (Query):</strong>  The query that filters the documents to aggregate over.</li>
<li><strong>agg (dict):</strong>  The aggregation specification as a Python dict.</li>
</ul>

<p>Returns a dict containing the aggregation results.</p>
</div>


                            </div>
                            <div id="Searcher.cardinality" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">cardinality</span><span class="signature pdoc-code condensed">(<span class="param"><span class="bp">self</span>, </span><span class="param"><span class="n">query</span><span class="p">:</span> <span class="n"><a href="#Query">Query</a></span>, </span><span class="param"><span class="n">field_name</span><span class="p">:</span> <span class="nb">str</span></span><span class="return-annotation">) -> <span class="nb">float</span>:</span></span>

        
    </div>
    <a class="headerlink" href="#Searcher.cardinality"></a>
    
            <div class="docstring"><p>Returns the cardinality of a query.</p>

<h6 id="arguments">Arguments:</h6>

<ul>
<li><strong>query (Query):</strong>  The query that will be used for the search.</li>
<li><strong>field_name (str):</strong>  The field for which to compute the cardinality.</li>
</ul>

<p>Returns the cardinality.</p>
</div>


                            </div>
                            <div id="Searcher.doc" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">doc</span><span class="signature pdoc-code multiline">(<span class="param">    <span class="bp">self</span>,</span><span class="param">    <span class="n">doc_address</span><span class="p">:</span> <span class="n"><a href="#DocAddress">DocAddress</a></span></span><span class="return-annotation">) -> <span class="n"><a href="#Document">Document</a></span>:</span></span>

        
    </div>
    <a class="headerlink" href="#Searcher.doc"></a>
    
            <div class="docstring"><p>Fetches a document from Tantivy's store given a DocAddress.</p>

<h6 id="arguments">Arguments:</h6>

<ul>
<li><strong>doc_address (DocAddress):</strong>  The DocAddress that is associated with
the document that we wish to fetch.</li>
</ul>

<p>Returns the Document, raises ValueError if the document can't be found.</p>
</div>


                            </div>
                            <div id="Searcher.doc_freq" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">doc_freq</span><span class="signature pdoc-code condensed">(<span class="param"><span class="bp">self</span>, </span><span class="param"><span class="n">field_name</span><span class="p">:</span> <span class="nb">str</span>, </span><span class="param"><span class="n">field_value</span><span class="p">:</span> <span class="n">Any</span></span><span class="return-annotation">) -> <span class="nb">int</span>:</span></span>

        
    </div>
    <a class="headerlink" href="#Searcher.doc_freq"></a>
    
            <div class="docstring"><p>Return the overall number of documents containing
the given term.</p>
</div>


                            </div>
                            <div id="Searcher.fast_field_values" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">fast_field_values</span><span class="signature pdoc-code multiline">(<span class="param">    <span class="bp">self</span>,</span><span class="param">    <span class="n">field_name</span><span class="p">:</span> <span class="nb">str</span>,</span><span class="param">    <span class="n">doc_addresses</span><span class="p">:</span> <span class="nb">list</span><span class="p">[</span><span class="n"><a href="#DocAddress">DocAddress</a></span><span class="p">]</span></span><span class="return-annotation">) -> <span class="nb">list</span><span class="p">[</span><span class="nb">int</span> <span class="o">|</span> <span class="nb">float</span> <span class="o">|</span> <span class="nb">bool</span> <span class="o">|</span> <span class="kc">None</span><span class="p">]</span>:</span></span>

        
    </div>
    <a class="headerlink" href="#Searcher.fast_field_values"></a>
    
            <div class="docstring"><p>Read a numeric fast field for a batch of DocAddresses without fetching
stored documents.</p>

<p>Fast fields are column-oriented and support O(1) random access by
segment-local DocId.  Use this instead of doc().to_dict()[field] when
you only need a single numeric field for many documents.</p>

<p>The field type is resolved from the schema automatically: u64 and i64
fields return Python int; f64 fields return Python float; bool fields
return Python bool.</p>

<h6 id="arguments">Arguments:</h6>

<ul>
<li><strong>field_name:</strong>  Name of a u64, i64, f64, or bool field declared with fast=True.</li>
<li><strong>doc_addresses:</strong>  List of DocAddress objects (e.g. from search().hits).</li>
</ul>

<h6 id="returns">Returns:</h6>

<blockquote>
  <p>A list of values in the same order as doc_addresses.
  None is returned for any address where the column is absent
  (e.g. a segment written before the field was added to the schema).</p>
</blockquote>

<h6 id="raises">Raises:</h6>

<ul>
<li><strong>ValueError:</strong>  if the field does not exist, is not a fast field, or
has an unsupported type (only u64, i64, f64, and bool are supported).</li>
</ul>
</div>


                            </div>
                            <div id="Searcher.num_docs" class="classattr">
                                <div class="attr variable">
            <span class="name">num_docs</span><span class="annotation">: int</span>

        
    </div>
    <a class="headerlink" href="#Searcher.num_docs"></a>
    
            <div class="docstring"><p>Returns the overall number of documents in the index.</p>
</div>


                            </div>
                            <div id="Searcher.num_segments" class="classattr">
                                <div class="attr variable">
            <span class="name">num_segments</span><span class="annotation">: int</span>

        
    </div>
    <a class="headerlink" href="#Searcher.num_segments"></a>
    
            <div class="docstring"><p>Returns the number of segments in the index.</p>
</div>


                            </div>
                            <div id="Searcher.search" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">search</span><span class="signature pdoc-code multiline">(<span class="param">    <span class="bp">self</span>,</span><span class="param">    <span class="n">query</span><span class="p">:</span> <span class="n"><a href="#Query">Query</a></span>,</span><span class="param">    <span class="n">limit</span><span class="p">:</span> <span class="nb">int</span> <span class="o">=</span> <span class="mi">10</span>,</span><span class="param">    <span class="n">count</span><span class="p">:</span> <span class="nb">bool</span> <span class="o">=</span> <span class="kc">True</span>,</span><span class="param">    <span class="n">order_by_field</span><span class="p">:</span> <span class="nb">str</span> <span class="o">|</span> <span class="kc">None</span> <span class="o">=</span> <span class="kc">None</span>,</span><span class="param">    <span class="n">offset</span><span class="p">:</span> <span class="nb">int</span> <span class="o">=</span> <span class="mi">0</span>,</span><span class="param">    <span class="n">order</span><span class="p">:</span> <span class="n"><a href="#Order">Order</a></span> <span class="o">=</span> <span class="o">&lt;</span><span class="n"><a href="#Order.Desc">Order.Desc</a></span><span class="p">:</span> <span class="mi">2</span><span class="o">&gt;</span>,</span><span class="param">    <span class="n">weight_by_field</span><span class="p">:</span> <span class="nb">str</span> <span class="o">|</span> <span class="kc">None</span> <span class="o">=</span> <span class="kc">None</span></span><span class="return-annotation">) -> <span class="n"><a href="#SearchResult">SearchResult</a></span>:</span></span>

        
    </div>
    <a class="headerlink" href="#Searcher.search"></a>
    
            <div class="docstring"><p>Search the index with the given query and collect results.</p>

<h6 id="arguments">Arguments:</h6>

<ul>
<li><strong>query (Query):</strong>  The query that will be used for the search.</li>
<li><strong>limit (int, optional):</strong>  The maximum number of search results to
return. Defaults to 10.</li>
<li><strong>count (bool, optional):</strong>  Should the number of documents that match
the query be returned as well. Defaults to true.</li>
<li><strong>order_by_field (str, optional):</strong>  Name of a field that the results
should be ordered by. The field must be declared as a fast field
when building the schema. Supported field types: Text, Unsigned,
Integer, Float, Boolean and Date.</li>
<li><strong>offset (int, optional):</strong>  The offset from which the results have
to be returned.</li>
<li><strong>order (Order, optional):</strong>  The order in which the results
should be sorted. If not specified, defaults to descending.</li>
<li><strong>weight_by_field (str, optional):</strong>  Name of a field that the results
should be weighted by. The field must be declared as a fast
field when building the schema. Note, this only works for
Float, Integer and Unsigned fields. The given field value is first
transformed using the formula <code>log2(2.0 + value)</code> and then
multiplied with the original score. This means that a weight field
value of 0.0 results in no change to the original score.
If the weight value is negative, it is treated as 0.0.</li>
</ul>

<p>Returns <code><a href="#SearchResult">SearchResult</a></code> object whose <code>hits</code> is a list of <code>(order_key,
DocAddress)</code> tuples. When no <code>order_by_field</code> is given, <code>order_key</code> is
a float score. When ordering by a field, <code>order_key</code> matches the
field's Python type (int, float, bool, or str), except for date fields
which return an int of nanoseconds since the epoch.</p>

<p>Raises a ValueError if there was an error with the search.</p>
</div>


                            </div>
                            <div id="Searcher.terms_with_prefix" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">terms_with_prefix</span><span class="signature pdoc-code multiline">(<span class="param">    <span class="bp">self</span>,</span><span class="param">    <span class="n">field_name</span><span class="p">:</span> <span class="nb">str</span>,</span><span class="param">    <span class="n">prefix</span><span class="p">:</span> <span class="nb">str</span>,</span><span class="param">    <span class="n">filter_query</span><span class="p">:</span> <span class="n"><a href="#Query">Query</a></span> <span class="o">|</span> <span class="kc">None</span> <span class="o">=</span> <span class="kc">None</span>,</span><span class="param">    <span class="n">limit</span><span class="p">:</span> <span class="nb">int</span> <span class="o">|</span> <span class="kc">None</span> <span class="o">=</span> <span class="kc">None</span></span><span class="return-annotation">) -> <span class="nb">list</span><span class="p">[</span><span class="nb">tuple</span><span class="p">[</span><span class="nb">str</span><span class="p">,</span> <span class="nb">int</span><span class="p">]]</span>:</span></span>

        
    </div>
    <a class="headerlink" href="#Searcher.terms_with_prefix"></a>
    
            <div class="docstring"><p>Walk the term dictionary for <code>field_name</code> and return all terms that
begin with <code>prefix</code>, together with their document frequencies.</p>

<h6 id="arguments">Arguments:</h6>

<ul>
<li><strong>field_name:</strong>  Name of an indexed text field in the schema.</li>
<li><strong>prefix:</strong>  Only terms beginning with this string are returned.
An empty string returns all terms in the field.</li>
<li><strong>filter_query:</strong>  Optional Query. When provided, each term's count
reflects only documents matched by the query (e.g. for
permission filtering). Counts are still summed across segments.</li>
<li><strong>limit:</strong>  If given, only the top-<code>limit</code> entries (by count) are returned.</li>
</ul>

<h6 id="returns">Returns:</h6>

<blockquote>
  <p><code>[(term, count), ...]</code> sorted by count descending, then
  alphabetically. Terms present in multiple segments have their
  counts summed.</p>
</blockquote>

<h6 id="raises">Raises:</h6>

<ul>
<li><strong>ValueError:</strong>  if the field does not exist or is not a text field.</li>
</ul>
</div>


                            </div>
                </section>
</main>

## SearchResult

<main class="pdoc">
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
            <span class="name">hits</span><span class="annotation">: list[tuple[typing.Any, <a href="#DocAddress">DocAddress</a>]]</span>

        
    </div>
    <a class="headerlink" href="#SearchResult.hits"></a>
    
            <div class="docstring"><p>The list of tuples that contains the scores and DocAddress of the
search results.</p>
</div>


                            </div>
                </section>
</main>

## Snippet

<main class="pdoc">
<section id="Snippet">
                    <div class="attr class">
            
    <span class="def">class</span>
    <span class="name">Snippet</span>:

        
    </div>
    <a class="headerlink" href="#Snippet"></a>
    
            <div class="docstring"><p>A fragment of a document with highlighted search terms.</p>

<p>Contains a text fragment (a window around the matched terms) and
the byte ranges within that fragment that matched the query.</p>
</div>


                            <div id="Snippet.fragment" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">fragment</span><span class="signature pdoc-code condensed">(<span class="param"><span class="bp">self</span></span><span class="return-annotation">) -> <span class="nb">str</span>:</span></span>

        
    </div>
    <a class="headerlink" href="#Snippet.fragment"></a>
    
            <div class="docstring"><p>Returns the text fragment that contains the highlighted terms.</p>
</div>


                            </div>
                            <div id="Snippet.highlighted" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">highlighted</span><span class="signature pdoc-code condensed">(<span class="param"><span class="bp">self</span></span><span class="return-annotation">) -> <span class="nb">list</span><span class="p">[</span><span class="n">tantivy</span><span class="o">.</span><span class="n">tantivy</span><span class="o">.</span><span class="n">Range</span><span class="p">]</span>:</span></span>

        
    </div>
    <a class="headerlink" href="#Snippet.highlighted"></a>
    
            <div class="docstring"><p>Returns the highlighted ranges within the fragment.</p>

<p>The offsets are relative to the string returned by <code><a href="#Snippet.fragment">fragment()</a></code>,
not the original document text.</p>
</div>


                            </div>
                            <div id="Snippet.to_html" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">to_html</span><span class="signature pdoc-code condensed">(<span class="param"><span class="bp">self</span></span><span class="return-annotation">) -> <span class="nb">str</span>:</span></span>

        
    </div>
    <a class="headerlink" href="#Snippet.to_html"></a>
    
            <div class="docstring"><p>Returns the fragment as HTML with matched terms wrapped in <code>&lt;b&gt;</code> tags.</p>
</div>


                            </div>
                </section>
</main>

## SnippetGenerator

<main class="pdoc">
<section id="SnippetGenerator">
                    <div class="attr class">
            
    <span class="def">class</span>
    <span class="name">SnippetGenerator</span>:

        
    </div>
    <a class="headerlink" href="#SnippetGenerator"></a>
    
    

                            <div id="SnippetGenerator.create" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">create</span><span class="signature pdoc-code multiline">(<span class="param">    <span class="n">searcher</span><span class="p">:</span> <span class="n"><a href="#Searcher">Searcher</a></span>,</span><span class="param">    <span class="n">query</span><span class="p">:</span> <span class="n"><a href="#Query">Query</a></span>,</span><span class="param">    <span class="n">schema</span><span class="p">:</span> <span class="n"><a href="#Schema">Schema</a></span>,</span><span class="param">    <span class="n">field_name</span><span class="p">:</span> <span class="nb">str</span></span><span class="return-annotation">) -> <span class="n"><a href="#SnippetGenerator">SnippetGenerator</a></span>:</span></span>

        
    </div>
    <a class="headerlink" href="#SnippetGenerator.create"></a>
    
            <div class="docstring"><p>The type of the None singleton.</p>
</div>


                            </div>
                            <div id="SnippetGenerator.set_max_num_chars" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">set_max_num_chars</span><span class="signature pdoc-code condensed">(<span class="param"><span class="bp">self</span>, </span><span class="param"><span class="n">max_num_chars</span><span class="p">:</span> <span class="nb">int</span></span><span class="return-annotation">) -> <span class="kc">None</span>:</span></span>

        
    </div>
    <a class="headerlink" href="#SnippetGenerator.set_max_num_chars"></a>
    
            <div class="docstring"><p>The type of the None singleton.</p>
</div>


                            </div>
                            <div id="SnippetGenerator.snippet_from_doc" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">snippet_from_doc</span><span class="signature pdoc-code condensed">(<span class="param"><span class="bp">self</span>, </span><span class="param"><span class="n">doc</span><span class="p">:</span> <span class="n"><a href="#Document">Document</a></span></span><span class="return-annotation">) -> <span class="n"><a href="#Snippet">Snippet</a></span>:</span></span>

        
    </div>
    <a class="headerlink" href="#SnippetGenerator.snippet_from_doc"></a>
    
            <div class="docstring"><p>The type of the None singleton.</p>
</div>


                            </div>
                </section>
</main>

## TextAnalyzer

<main class="pdoc">
<section id="TextAnalyzer">
                    <div class="attr class">
            
    <span class="def">class</span>
    <span class="name">TextAnalyzer</span>:

        
    </div>
    <a class="headerlink" href="#TextAnalyzer"></a>
    
            <div class="docstring"><p>Tantivy's TextAnalyzer</p>

<p>Do not instantiate this class directly.
Use the <code><a href="#TextAnalyzerBuilder">TextAnalyzerBuilder</a></code> class instead.</p>
</div>


                            <div id="TextAnalyzer.analyze" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">analyze</span><span class="signature pdoc-code condensed">(<span class="param"><span class="bp">self</span>, </span><span class="param"><span class="n">text</span><span class="p">:</span> <span class="nb">str</span></span><span class="return-annotation">) -> <span class="nb">list</span><span class="p">[</span><span class="nb">str</span><span class="p">]</span>:</span></span>

        
    </div>
    <a class="headerlink" href="#TextAnalyzer.analyze"></a>
    
            <div class="docstring"><p>Tokenize a string
Args:</p>

<ul>
<li>text (string): text to tokenize.
Returns:</li>
<li>list(string): a list of tokens/words.</li>
</ul>
</div>


                            </div>
                </section>
</main>

## TextAnalyzerBuilder

<main class="pdoc">
<section id="TextAnalyzerBuilder">
                    <div class="attr class">
            
    <span class="def">class</span>
    <span class="name">TextAnalyzerBuilder</span>:

        
    </div>
    <a class="headerlink" href="#TextAnalyzerBuilder"></a>
    
            <div class="docstring"><p>Tantivy's TextAnalyzerBuilder</p>

<h1 id="example">Example</h1>

<div class="pdoc-code codehilite">
<pre><span></span><code><span class="n">my_analyzer</span><span class="p">:</span> <span class="n">TextAnalyzer</span> <span class="o">=</span> <span class="p">(</span>
    <span class="n">TextAnalyzerBuilder</span><span class="p">(</span><span class="n"><a href="#Tokenizer.simple">Tokenizer.simple</a></span><span class="p">())</span>
    <span class="o">.</span><span class="n">filter</span><span class="p">(</span><span class="n"><a href="#Filter.lowercase">Filter.lowercase</a></span><span class="p">())</span>
    <span class="o">.</span><span class="n">filter</span><span class="p">(</span><span class="n">Filter</span><span class="o">.</span><span class="n">ngram</span><span class="p">())</span>
    <span class="o">.</span><span class="n">build</span><span class="p">()</span>
<span class="p">)</span>
</code></pre>
</div>

<p><a href="https://docs.rs/tantivy/latest/tantivy/tokenizer/struct.TextAnalyzerBuilder.html">https://docs.rs/tantivy/latest/tantivy/tokenizer/struct<a href="#TextAnalyzerBuilder">.TextAnalyzerBuilder</a>.html</a></p>
</div>


                            <div id="TextAnalyzerBuilder.__init__" class="classattr">
                                <div class="attr function">
            
        <span class="name">TextAnalyzerBuilder</span><span class="signature pdoc-code condensed">(<span class="param"><span class="n">tokenizer</span><span class="p">:</span> <span class="n"><a href="#Tokenizer">Tokenizer</a></span></span>)</span>

        
    </div>
    <a class="headerlink" href="#TextAnalyzerBuilder.__init__"></a>
    
    

                            </div>
                            <div id="TextAnalyzerBuilder.build" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">build</span><span class="signature pdoc-code condensed">(<span class="param"><span class="bp">self</span></span><span class="return-annotation">) -> <span class="n"><a href="#TextAnalyzer">TextAnalyzer</a></span>:</span></span>

        
    </div>
    <a class="headerlink" href="#TextAnalyzerBuilder.build"></a>
    
            <div class="docstring"><p>Build final TextAnalyzer object.</p>

<p>Returns:</p>

<ul>
<li>TextAnalyzer with tokenizer and filters baked in.</li>
</ul>

<p>Tip: TextAnalyzer's <code>analyze(text) -&gt; tokens</code> method lets you
easily check if your analyzer is working as expected.</p>
</div>


                            </div>
                            <div id="TextAnalyzerBuilder.filter" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">filter</span><span class="signature pdoc-code multiline">(<span class="param">    <span class="bp">self</span>,</span><span class="param">    <span class="nb">filter</span><span class="p">:</span> <span class="n"><a href="#Filter">Filter</a></span></span><span class="return-annotation">) -> <span class="n"><a href="#TextAnalyzerBuilder">TextAnalyzerBuilder</a></span>:</span></span>

        
    </div>
    <a class="headerlink" href="#TextAnalyzerBuilder.filter"></a>
    
            <div class="docstring"><p>Add filter to the builder.</p>

<p>Args:</p>

<ul>
<li>filter (Filter): a Filter object.
Returns:</li>
<li>TextAnalyzerBuilder: A new instance of the builder</li>
</ul>

<p>Note: The builder is _not_ mutated in-place.</p>
</div>


                            </div>
                </section>
</main>

## Tokenizer

<main class="pdoc">
<section id="Tokenizer">
                    <div class="attr class">
            
    <span class="def">class</span>
    <span class="name">Tokenizer</span>:

        
    </div>
    <a class="headerlink" href="#Tokenizer"></a>
    
            <div class="docstring"><p>All Tantivy's built-in tokenizers in one place.
Each static method, e.g. <a href="#Tokenizer.simple">Tokenizer.simple()</a>,
creates a wrapper around a Tantivy tokenizer.</p>

<h2 id="example">Example:</h2>

<div class="pdoc-code codehilite">
<pre><span></span><code><span class="n">tokenizer</span> <span class="o">=</span> <span class="n"><a href="#Tokenizer.regex">Tokenizer.regex</a></span><span class="p">(</span><span class="sa">r</span><span class="s2">&quot;\w+&quot;</span><span class="p">)</span>
</code></pre>
</div>

<h2 id="usage">Usage</h2>

<p>In general, tokenizer objects' only reason
for existing is to be passed to
TextAnalyzerBuilder(tokenizer=<tokenizer>)</p>

<p><a href="https://docs.rs/tantivy/latest/tantivy/tokenizer/index.html">https://docs.rs/tantivy/latest/tantivy/tokenizer/index.html</a></p>
</div>


                            <div id="Tokenizer.facet" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">facet</span><span class="signature pdoc-code condensed">(<span class="return-annotation">) -> <span class="n"><a href="#Tokenizer">Tokenizer</a></span>:</span></span>

        
    </div>
    <a class="headerlink" href="#Tokenizer.facet"></a>
    
            <div class="docstring"><p>FacetTokenizer</p>
</div>


                            </div>
                            <div id="Tokenizer.ngram" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">ngram</span><span class="signature pdoc-code multiline">(<span class="param">    <span class="n">min_gram</span><span class="p">:</span> <span class="nb">int</span> <span class="o">=</span> <span class="mi">2</span>,</span><span class="param">    <span class="n">max_gram</span><span class="p">:</span> <span class="nb">int</span> <span class="o">=</span> <span class="mi">3</span>,</span><span class="param">    <span class="n">prefix_only</span><span class="p">:</span> <span class="nb">bool</span> <span class="o">=</span> <span class="kc">False</span></span><span class="return-annotation">) -> <span class="n"><a href="#Tokenizer">Tokenizer</a></span>:</span></span>

        
    </div>
    <a class="headerlink" href="#Tokenizer.ngram"></a>
    
            <div class="docstring"><p>NgramTokenizer</p>

<p>Args:</p>

<ul>
<li>min_gram (int): Minimum character length of each ngram.</li>
<li>max_gram (int): Maximum character length of each ngram.</li>
<li>prefix_only (bool, optional): If true, ngrams must count from the start of the word.</li>
</ul>
</div>


                            </div>
                            <div id="Tokenizer.raw" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">raw</span><span class="signature pdoc-code condensed">(<span class="return-annotation">) -> <span class="n"><a href="#Tokenizer">Tokenizer</a></span>:</span></span>

        
    </div>
    <a class="headerlink" href="#Tokenizer.raw"></a>
    
            <div class="docstring"><p>Raw Tokenizer</p>
</div>


                            </div>
                            <div id="Tokenizer.regex" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">regex</span><span class="signature pdoc-code condensed">(<span class="param"><span class="n">pattern</span><span class="p">:</span> <span class="nb">str</span></span><span class="return-annotation">) -> <span class="n"><a href="#Tokenizer">Tokenizer</a></span>:</span></span>

        
    </div>
    <a class="headerlink" href="#Tokenizer.regex"></a>
    
            <div class="docstring"><p>Regextokenizer</p>
</div>


                            </div>
                            <div id="Tokenizer.simple" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">simple</span><span class="signature pdoc-code condensed">(<span class="return-annotation">) -> <span class="n"><a href="#Tokenizer">Tokenizer</a></span>:</span></span>

        
    </div>
    <a class="headerlink" href="#Tokenizer.simple"></a>
    
            <div class="docstring"><p>SimpleTokenizer</p>
</div>


                            </div>
                            <div id="Tokenizer.whitespace" class="classattr">
                                <div class="attr function">
            
        <span class="def">def</span>
        <span class="name">whitespace</span><span class="signature pdoc-code condensed">(<span class="return-annotation">) -> <span class="n"><a href="#Tokenizer">Tokenizer</a></span>:</span></span>

        
    </div>
    <a class="headerlink" href="#Tokenizer.whitespace"></a>
    
            <div class="docstring"><p>WhitespaceTokenizer</p>
</div>


                            </div>
                </section>
</main>

<style>pre{line-height:125%;}span.linenos{color:inherit; background-color:transparent; padding-left:5px; padding-right:20px;}.pdoc-code .hll{background-color:#ffffcc}.pdoc-code{background:#f8f8f8;}.pdoc-code .c{color:#3D7B7B; font-style:italic}.pdoc-code .err{border:1px solid #FF0000}.pdoc-code .k{color:#008000; font-weight:bold}.pdoc-code .o{color:#666666}.pdoc-code .ch{color:#3D7B7B; font-style:italic}.pdoc-code .cm{color:#3D7B7B; font-style:italic}.pdoc-code .cp{color:#9C6500}.pdoc-code .cpf{color:#3D7B7B; font-style:italic}.pdoc-code .c1{color:#3D7B7B; font-style:italic}.pdoc-code .cs{color:#3D7B7B; font-style:italic}.pdoc-code .gd{color:#A00000}.pdoc-code .ge{font-style:italic}.pdoc-code .gr{color:#E40000}.pdoc-code .gh{color:#000080; font-weight:bold}.pdoc-code .gi{color:#008400}.pdoc-code .go{color:#717171}.pdoc-code .gp{color:#000080; font-weight:bold}.pdoc-code .gs{font-weight:bold}.pdoc-code .gu{color:#800080; font-weight:bold}.pdoc-code .gt{color:#0044DD}.pdoc-code .kc{color:#008000; font-weight:bold}.pdoc-code .kd{color:#008000; font-weight:bold}.pdoc-code .kn{color:#008000; font-weight:bold}.pdoc-code .kp{color:#008000}.pdoc-code .kr{color:#008000; font-weight:bold}.pdoc-code .kt{color:#B00040}.pdoc-code .m{color:#666666}.pdoc-code .s{color:#BA2121}.pdoc-code .na{color:#687822}.pdoc-code .nb{color:#008000}.pdoc-code .nc{color:#0000FF; font-weight:bold}.pdoc-code .no{color:#880000}.pdoc-code .nd{color:#AA22FF}.pdoc-code .ni{color:#717171; font-weight:bold}.pdoc-code .ne{color:#CB3F38; font-weight:bold}.pdoc-code .nf{color:#0000FF}.pdoc-code .nl{color:#767600}.pdoc-code .nn{color:#0000FF; font-weight:bold}.pdoc-code .nt{color:#008000; font-weight:bold}.pdoc-code .nv{color:#19177C}.pdoc-code .ow{color:#AA22FF; font-weight:bold}.pdoc-code .w{color:#bbbbbb}.pdoc-code .mb{color:#666666}.pdoc-code .mf{color:#666666}.pdoc-code .mh{color:#666666}.pdoc-code .mi{color:#666666}.pdoc-code .mo{color:#666666}.pdoc-code .sa{color:#BA2121}.pdoc-code .sb{color:#BA2121}.pdoc-code .sc{color:#BA2121}.pdoc-code .dl{color:#BA2121}.pdoc-code .sd{color:#BA2121; font-style:italic}.pdoc-code .s2{color:#BA2121}.pdoc-code .se{color:#AA5D1F; font-weight:bold}.pdoc-code .sh{color:#BA2121}.pdoc-code .si{color:#A45A77; font-weight:bold}.pdoc-code .sx{color:#008000}.pdoc-code .sr{color:#A45A77}.pdoc-code .s1{color:#BA2121}.pdoc-code .ss{color:#19177C}.pdoc-code .bp{color:#008000}.pdoc-code .fm{color:#0000FF}.pdoc-code .vc{color:#19177C}.pdoc-code .vg{color:#19177C}.pdoc-code .vi{color:#19177C}.pdoc-code .vm{color:#19177C}.pdoc-code .il{color:#666666}</style><style>:root{--pdoc-background:#fff;}.pdoc{--text:#212529;--muted:#6c757d;--link:#3660a5;--link-hover:#1659c5;--code:#f8f8f8;--active:#fff598;--accent:#eee;--accent2:#c1c1c1;--nav-hover:rgba(255, 255, 255, 0.5);--name:#0066BB;--def:#008800;--annotation:#007020;}</style><style>.pdoc{color:var(--text);box-sizing:border-box;line-height:1.5;background:none;}.pdoc .pdoc-button{cursor:pointer;display:inline-block;border:solid black 1px;border-radius:2px;font-size:.75rem;padding:calc(0.5em - 1px) 1em;transition:100ms all;}.pdoc .alert{padding:1rem 1rem 1rem calc(1.5rem + 24px);border:1px solid transparent;border-radius:.25rem;background-repeat:no-repeat;background-position:.75rem center;margin-bottom:1rem;}.pdoc .alert > em{display:none;}.pdoc .alert > *:last-child{margin-bottom:0;}.pdoc .alert.note{color:#084298;background-color:#cfe2ff;border-color:#b6d4fe;background-image:url("data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A//www.w3.org/2000/svg%22%20width%3D%2224%22%20height%3D%2224%22%20fill%3D%22%23084298%22%20viewBox%3D%220%200%2016%2016%22%3E%3Cpath%20d%3D%22M8%2016A8%208%200%201%200%208%200a8%208%200%200%200%200%2016zm.93-9.412-1%204.705c-.07.34.029.533.304.533.194%200%20.487-.07.686-.246l-.088.416c-.287.346-.92.598-1.465.598-.703%200-1.002-.422-.808-1.319l.738-3.468c.064-.293.006-.399-.287-.47l-.451-.081.082-.381%202.29-.287zM8%205.5a1%201%200%201%201%200-2%201%201%200%200%201%200%202z%22/%3E%3C/svg%3E");}.pdoc .alert.tip{color:#0a3622;background-color:#d1e7dd;border-color:#a3cfbb;background-image:url("data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A//www.w3.org/2000/svg%22%20width%3D%2224%22%20height%3D%2224%22%20fill%3D%22%230a3622%22%20viewBox%3D%220%200%2016%2016%22%3E%3Cpath%20d%3D%22M2%206a6%206%200%201%201%2010.174%204.31c-.203.196-.359.4-.453.619l-.762%201.769A.5.5%200%200%201%2010.5%2013a.5.5%200%200%201%200%201%20.5.5%200%200%201%200%201l-.224.447a1%201%200%200%201-.894.553H6.618a1%201%200%200%201-.894-.553L5.5%2015a.5.5%200%200%201%200-1%20.5.5%200%200%201%200-1%20.5.5%200%200%201-.46-.302l-.761-1.77a2%202%200%200%200-.453-.618A5.98%205.98%200%200%201%202%206m6-5a5%205%200%200%200-3.479%208.592c.263.254.514.564.676.941L5.83%2012h4.342l.632-1.467c.162-.377.413-.687.676-.941A5%205%200%200%200%208%201%22/%3E%3C/svg%3E");}.pdoc .alert.important{color:#055160;background-color:#cff4fc;border-color:#9eeaf9;background-image:url("data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A//www.w3.org/2000/svg%22%20width%3D%2224%22%20height%3D%2224%22%20fill%3D%22%23055160%22%20viewBox%3D%220%200%2016%2016%22%3E%3Cpath%20d%3D%22M2%200a2%202%200%200%200-2%202v12a2%202%200%200%200%202%202h12a2%202%200%200%200%202-2V2a2%202%200%200%200-2-2zm6%204c.535%200%20.954.462.9.995l-.35%203.507a.552.552%200%200%201-1.1%200L7.1%204.995A.905.905%200%200%201%208%204m.002%206a1%201%200%201%201%200%202%201%201%200%200%201%200-2%22/%3E%3C/svg%3E");}.pdoc .alert.warning{color:#664d03;background-color:#fff3cd;border-color:#ffecb5;background-image:url("data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A//www.w3.org/2000/svg%22%20width%3D%2224%22%20height%3D%2224%22%20fill%3D%22%23664d03%22%20viewBox%3D%220%200%2016%2016%22%3E%3Cpath%20d%3D%22M8.982%201.566a1.13%201.13%200%200%200-1.96%200L.165%2013.233c-.457.778.091%201.767.98%201.767h13.713c.889%200%201.438-.99.98-1.767L8.982%201.566zM8%205c.535%200%20.954.462.9.995l-.35%203.507a.552.552%200%200%201-1.1%200L7.1%205.995A.905.905%200%200%201%208%205zm.002%206a1%201%200%201%201%200%202%201%201%200%200%201%200-2z%22/%3E%3C/svg%3E");}.pdoc .alert.caution{color:#842029;background-color:#f8d7da;border-color:#f5c2c7;background-image:url("data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A//www.w3.org/2000/svg%22%20width%3D%2224%22%20height%3D%2224%22%20fill%3D%22%23842029%22%20viewBox%3D%220%200%2016%2016%22%3E%3Cpath%20d%3D%22M11.46.146A.5.5%200%200%200%2011.107%200H4.893a.5.5%200%200%200-.353.146L.146%204.54A.5.5%200%200%200%200%204.893v6.214a.5.5%200%200%200%20.146.353l4.394%204.394a.5.5%200%200%200%20.353.146h6.214a.5.5%200%200%200%20.353-.146l4.394-4.394a.5.5%200%200%200%20.146-.353V4.893a.5.5%200%200%200-.146-.353zM8%204c.535%200%20.954.462.9.995l-.35%203.507a.552.552%200%200%201-1.1%200L7.1%204.995A.905.905%200%200%201%208%204m.002%206a1%201%200%201%201%200%202%201%201%200%200%201%200-2%22/%3E%3C/svg%3E");}.pdoc .alert.danger{color:#842029;background-color:#f8d7da;border-color:#f5c2c7;background-image:url("data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A//www.w3.org/2000/svg%22%20width%3D%2224%22%20height%3D%2224%22%20fill%3D%22%23842029%22%20viewBox%3D%220%200%2016%2016%22%3E%3Cpath%20d%3D%22M5.52.359A.5.5%200%200%201%206%200h4a.5.5%200%200%201%20.474.658L8.694%206H12.5a.5.5%200%200%201%20.395.807l-7%209a.5.5%200%200%201-.873-.454L6.823%209.5H3.5a.5.5%200%200%201-.48-.641l2.5-8.5z%22/%3E%3C/svg%3E");}.pdoc .visually-hidden{position:absolute !important;width:1px !important;height:1px !important;padding:0 !important;margin:-1px !important;overflow:hidden !important;clip:rect(0, 0, 0, 0) !important;white-space:nowrap !important;border:0 !important;}.pdoc h1, .pdoc h2, .pdoc h3{font-weight:300;margin:.3em 0;padding:.2em 0;}.pdoc > section:not(.module-info) h1{font-size:1.5rem;font-weight:500;}.pdoc > section:not(.module-info) h2{font-size:1.4rem;font-weight:500;}.pdoc > section:not(.module-info) h3{font-size:1.3rem;font-weight:500;}.pdoc > section:not(.module-info) h4{font-size:1.2rem;}.pdoc > section:not(.module-info) h5{font-size:1.1rem;}.pdoc a{text-decoration:none;color:var(--link);}.pdoc a:hover{color:var(--link-hover);}.pdoc blockquote{margin-left:2rem;}.pdoc pre{border-top:1px solid var(--accent2);border-bottom:1px solid var(--accent2);margin-top:0;margin-bottom:1em;padding:.5rem 0 .5rem .5rem;overflow-x:auto;background-color:var(--code);}.pdoc code{color:var(--text);padding:.2em .4em;margin:0;font-size:85%;background-color:var(--accent);border-radius:6px;}.pdoc a > code{color:inherit;}.pdoc pre > code{display:inline-block;font-size:inherit;background:none;border:none;padding:0;}.pdoc > section:not(.module-info){margin-bottom:1.5rem;}.pdoc .modulename{margin-top:0;font-weight:bold;}.pdoc .modulename a{color:var(--link);transition:100ms all;}.pdoc .git-button{float:right;border:solid var(--link) 1px;}.pdoc .git-button:hover{background-color:var(--link);color:var(--pdoc-background);}.view-source-toggle-state,.view-source-toggle-state ~ .pdoc-code{display:none;}.view-source-toggle-state:checked ~ .pdoc-code{display:block;}.view-source-button{display:inline-block;float:right;font-size:.75rem;line-height:1.5rem;color:var(--muted);padding:0 .4rem 0 1.3rem;cursor:pointer;text-indent:-2px;}.view-source-button > span{visibility:hidden;}.module-info .view-source-button{float:none;display:flex;justify-content:flex-end;margin:-1.2rem .4rem -.2rem 0;}.view-source-button::before{position:absolute;content:"View Source";display:list-item;list-style-type:disclosure-closed;}.view-source-toggle-state:checked ~ .attr .view-source-button::before,.view-source-toggle-state:checked ~ .view-source-button::before{list-style-type:disclosure-open;}.pdoc .docstring{margin-bottom:1.5rem;}.pdoc section:not(.module-info) .docstring{margin-left:clamp(0rem, 5vw - 2rem, 1rem);}.pdoc .docstring .pdoc-code{margin-left:1em;margin-right:1em;}.pdoc h1:target,.pdoc h2:target,.pdoc h3:target,.pdoc h4:target,.pdoc h5:target,.pdoc h6:target,.pdoc .pdoc-code > pre > span:target{background-color:var(--active);box-shadow:-1rem 0 0 0 var(--active);}.pdoc .pdoc-code > pre > span:target{display:block;}.pdoc div:target > .attr,.pdoc section:target > .attr,.pdoc dd:target > a{background-color:var(--active);}.pdoc *{scroll-margin:2rem;}.pdoc .pdoc-code .linenos{user-select:none;}.pdoc .attr:hover{filter:contrast(0.95);}.pdoc section, .pdoc .classattr{position:relative;}.pdoc .headerlink{--width:clamp(1rem, 3vw, 2rem);position:absolute;top:0;left:calc(0rem - var(--width));transition:all 100ms ease-in-out;opacity:0;}.pdoc .headerlink::before{content:"#";display:block;text-align:center;width:var(--width);height:2.3rem;line-height:2.3rem;font-size:1.5rem;}.pdoc .attr:hover ~ .headerlink,.pdoc *:target > .headerlink,.pdoc .headerlink:hover{opacity:1;}.pdoc .attr{display:block;margin:.5rem 0 .5rem;padding:.4rem .4rem .4rem 1rem;background-color:var(--accent);overflow-x:auto;}.pdoc .classattr{margin-left:2rem;}.pdoc .decorator-deprecated{color:#842029;}.pdoc .decorator-deprecated ~ span{filter:grayscale(1) opacity(0.8);}.pdoc .name{color:var(--name);font-weight:bold;}.pdoc .def{color:var(--def);font-weight:bold;}.pdoc .signature{background-color:transparent;}.pdoc .param, .pdoc .return-annotation{white-space:pre;}.pdoc .signature.multiline .param{display:block;}.pdoc .signature.condensed .param{display:inline-block;}.pdoc .annotation{color:var(--annotation);}.pdoc .view-value-toggle-state,.pdoc .view-value-toggle-state ~ .default_value{display:none;}.pdoc .view-value-toggle-state:checked ~ .default_value{display:inherit;}.pdoc .view-value-button{font-size:.5rem;vertical-align:middle;border-style:dashed;margin-top:-0.1rem;}.pdoc .view-value-button:hover{background:white;}.pdoc .view-value-button::before{content:"show";text-align:center;width:2.2em;display:inline-block;}.pdoc .view-value-toggle-state:checked ~ .view-value-button::before{content:"hide";}.pdoc .inherited{margin-left:2rem;}.pdoc .inherited dt{font-weight:700;}.pdoc .inherited dt, .pdoc .inherited dd{display:inline;margin-left:0;margin-bottom:.5rem;}.pdoc .inherited dd:not(:last-child):after{content:", ";}.pdoc .inherited .class:before{content:"class ";}.pdoc .inherited .function a:after{content:"()";}.pdoc .search-result .docstring{overflow:auto;max-height:25vh;}.pdoc .search-result.focused > .attr{background-color:var(--active);}.pdoc .attribution{margin-top:2rem;display:block;opacity:0.5;transition:all 200ms;filter:grayscale(100%);}.pdoc .attribution:hover{opacity:1;filter:grayscale(0%);}.pdoc .attribution img{margin-left:5px;height:27px;vertical-align:bottom;width:50px;transition:all 200ms;}.pdoc table{display:block;width:max-content;max-width:100%;overflow:auto;margin-bottom:1rem;}.pdoc table th{font-weight:600;}.pdoc table th, .pdoc table td{padding:6px 13px;border:1px solid var(--accent2);}</style><style>.pdoc pre,.pdoc code,.pdoc .signature{font-family:SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono","Courier New", Courier, monospace;font-size:12px;line-height:1.4;}</style>
