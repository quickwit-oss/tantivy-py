---
title: tantivy.credential_provider
---

# tantivy.credential_provider

<main class="pdoc">
<section class="module-info">
                    

                        <div class="docstring"><p>Credential providers for indexes stored in S3, Google Cloud Storage or Azure.</p>

<p>Pass one as <code>credential_provider</code> to <code>Index</code>, <code>Index.open</code>,
<code>Index.exists</code> or <code>Index.is_compatible</code>::</p>

<pre><code>from tantivy import CredentialProviderAWS, Index

index = Index.open(
    "s3://bucket/index",
    credential_provider=CredentialProviderAWS(profile_name="prod"),
)
</code></pre>

<p>Any callable taking no arguments and returning <code>(credentials, expiry)</code> also
works, where <code>credentials</code> is a <code>dict[str, str]</code> and <code>expiry</code> is when
they stop working, as seconds since the Unix epoch, or <code>None</code> if they never
expire. tantivy calls it again shortly before <code>expiry</code>. The keys are:</p>

<ul>
<li>S3: <code>aws_access_key_id</code>, <code>aws_secret_access_key</code>, and optionally
<code>aws_session_token</code>.</li>
<li>GCS: <code>bearer_token</code>.</li>
<li>Azure: one of <code>bearer_token</code>, <code>account_key</code> or <code>sas_token</code>.</li>
</ul>

<p>Adapted from Polars (https://github.com/pola-rs/polars), file
<code>py-polars/src/polars/io/cloud/credential_provider/_providers.py</code>.
Copyright (c) 2025 Ritchie Vink, MIT License; see <code>licenses/polars-LICENSE</code>
in the tantivy-py source distribution. Differences from Polars: no Polars
logging or unstable-API warnings, cached credentials are refreshed five
minutes before they expire rather than at expiry, and <code><a href="#CredentialProviderAWS">CredentialProviderAWS</a></code>
reports its profile's region through <code>_storage_options()</code>.</p>
</div>

                        <input id="mod-credential_provider-view-source" class="view-source-toggle-state" type="checkbox" aria-hidden="true" tabindex="-1">

                        <label class="view-source-button" for="mod-credential_provider-view-source"><span>View Source</span></label>

                        <div class="pdoc-code codehilite"><pre><span></span><span id="L-1"><a href="#L-1"><span class="linenos">  1</span></a><span class="sd">"""Credential providers for indexes stored in S3, Google Cloud Storage or Azure.</span>
</span><span id="L-2"><a href="#L-2"><span class="linenos">  2</span></a>
</span><span id="L-3"><a href="#L-3"><span class="linenos">  3</span></a><span class="sd">Pass one as ``credential_provider`` to ``Index``, ``Index.open``,</span>
</span><span id="L-4"><a href="#L-4"><span class="linenos">  4</span></a><span class="sd">``Index.exists`` or ``Index.is_compatible``::</span>
</span><span id="L-5"><a href="#L-5"><span class="linenos">  5</span></a>
</span><span id="L-6"><a href="#L-6"><span class="linenos">  6</span></a><span class="sd">    from tantivy import CredentialProviderAWS, Index</span>
</span><span id="L-7"><a href="#L-7"><span class="linenos">  7</span></a>
</span><span id="L-8"><a href="#L-8"><span class="linenos">  8</span></a><span class="sd">    index = Index.open(</span>
</span><span id="L-9"><a href="#L-9"><span class="linenos">  9</span></a><span class="sd">        "s3://bucket/index",</span>
</span><span id="L-10"><a href="#L-10"><span class="linenos"> 10</span></a><span class="sd">        credential_provider=CredentialProviderAWS(profile_name="prod"),</span>
</span><span id="L-11"><a href="#L-11"><span class="linenos"> 11</span></a><span class="sd">    )</span>
</span><span id="L-12"><a href="#L-12"><span class="linenos"> 12</span></a>
</span><span id="L-13"><a href="#L-13"><span class="linenos"> 13</span></a><span class="sd">Any callable taking no arguments and returning ``(credentials, expiry)`` also</span>
</span><span id="L-14"><a href="#L-14"><span class="linenos"> 14</span></a><span class="sd">works, where ``credentials`` is a ``dict[str, str]`` and ``expiry`` is when</span>
</span><span id="L-15"><a href="#L-15"><span class="linenos"> 15</span></a><span class="sd">they stop working, as seconds since the Unix epoch, or ``None`` if they never</span>
</span><span id="L-16"><a href="#L-16"><span class="linenos"> 16</span></a><span class="sd">expire. tantivy calls it again shortly before ``expiry``. The keys are:</span>
</span><span id="L-17"><a href="#L-17"><span class="linenos"> 17</span></a>
</span><span id="L-18"><a href="#L-18"><span class="linenos"> 18</span></a><span class="sd">- S3: ``aws_access_key_id``, ``aws_secret_access_key``, and optionally</span>
</span><span id="L-19"><a href="#L-19"><span class="linenos"> 19</span></a><span class="sd">  ``aws_session_token``.</span>
</span><span id="L-20"><a href="#L-20"><span class="linenos"> 20</span></a><span class="sd">- GCS: ``bearer_token``.</span>
</span><span id="L-21"><a href="#L-21"><span class="linenos"> 21</span></a><span class="sd">- Azure: one of ``bearer_token``, ``account_key`` or ``sas_token``.</span>
</span><span id="L-22"><a href="#L-22"><span class="linenos"> 22</span></a>
</span><span id="L-23"><a href="#L-23"><span class="linenos"> 23</span></a><span class="sd">Adapted from Polars (https://github.com/pola-rs/polars), file</span>
</span><span id="L-24"><a href="#L-24"><span class="linenos"> 24</span></a><span class="sd">``py-polars/src/polars/io/cloud/credential_provider/_providers.py``.</span>
</span><span id="L-25"><a href="#L-25"><span class="linenos"> 25</span></a><span class="sd">Copyright (c) 2025 Ritchie Vink, MIT License; see ``licenses/polars-LICENSE``</span>
</span><span id="L-26"><a href="#L-26"><span class="linenos"> 26</span></a><span class="sd">in the tantivy-py source distribution. Differences from Polars: no Polars</span>
</span><span id="L-27"><a href="#L-27"><span class="linenos"> 27</span></a><span class="sd">logging or unstable-API warnings, cached credentials are refreshed five</span>
</span><span id="L-28"><a href="#L-28"><span class="linenos"> 28</span></a><span class="sd">minutes before they expire rather than at expiry, and ``CredentialProviderAWS``</span>
</span><span id="L-29"><a href="#L-29"><span class="linenos"> 29</span></a><span class="sd">reports its profile's region through ``_storage_options()``.</span>
</span><span id="L-30"><a href="#L-30"><span class="linenos"> 30</span></a><span class="sd">"""</span>
</span><span id="L-31"><a href="#L-31"><span class="linenos"> 31</span></a>
</span><span id="L-32"><a href="#L-32"><span class="linenos"> 32</span></a><span class="kn">from</span><span class="w"> </span><span class="nn">__future__</span><span class="w"> </span><span class="kn">import</span> <span class="n">annotations</span>
</span><span id="L-33"><a href="#L-33"><span class="linenos"> 33</span></a>
</span><span id="L-34"><a href="#L-34"><span class="linenos"> 34</span></a><span class="kn">import</span><span class="w"> </span><span class="nn">abc</span>
</span><span id="L-35"><a href="#L-35"><span class="linenos"> 35</span></a><span class="kn">import</span><span class="w"> </span><span class="nn">importlib.util</span>
</span><span id="L-36"><a href="#L-36"><span class="linenos"> 36</span></a><span class="kn">import</span><span class="w"> </span><span class="nn">threading</span>
</span><span id="L-37"><a href="#L-37"><span class="linenos"> 37</span></a><span class="kn">import</span><span class="w"> </span><span class="nn">time</span>
</span><span id="L-38"><a href="#L-38"><span class="linenos"> 38</span></a><span class="kn">from</span><span class="w"> </span><span class="nn">collections.abc</span><span class="w"> </span><span class="kn">import</span> <span class="n">Callable</span>
</span><span id="L-39"><a href="#L-39"><span class="linenos"> 39</span></a><span class="kn">from</span><span class="w"> </span><span class="nn">datetime</span><span class="w"> </span><span class="kn">import</span> <span class="n">datetime</span><span class="p">,</span> <span class="n">timezone</span>
</span><span id="L-40"><a href="#L-40"><span class="linenos"> 40</span></a><span class="kn">from</span><span class="w"> </span><span class="nn">typing</span><span class="w"> </span><span class="kn">import</span> <span class="n">Any</span><span class="p">,</span> <span class="n">TypedDict</span><span class="p">,</span> <span class="n">Union</span>
</span><span id="L-41"><a href="#L-41"><span class="linenos"> 41</span></a>
</span><span id="L-42"><a href="#L-42"><span class="linenos"> 42</span></a><span class="n">__all__</span> <span class="o">=</span> <span class="p">[</span>
</span><span id="L-43"><a href="#L-43"><span class="linenos"> 43</span></a>    <span class="s2">"CredentialProvider"</span><span class="p">,</span>
</span><span id="L-44"><a href="#L-44"><span class="linenos"> 44</span></a>    <span class="s2">"CredentialProviderAWS"</span><span class="p">,</span>
</span><span id="L-45"><a href="#L-45"><span class="linenos"> 45</span></a>    <span class="s2">"CredentialProviderAzure"</span><span class="p">,</span>
</span><span id="L-46"><a href="#L-46"><span class="linenos"> 46</span></a>    <span class="s2">"CredentialProviderFunction"</span><span class="p">,</span>
</span><span id="L-47"><a href="#L-47"><span class="linenos"> 47</span></a>    <span class="s2">"CredentialProviderFunctionReturn"</span><span class="p">,</span>
</span><span id="L-48"><a href="#L-48"><span class="linenos"> 48</span></a>    <span class="s2">"CredentialProviderGCP"</span><span class="p">,</span>
</span><span id="L-49"><a href="#L-49"><span class="linenos"> 49</span></a><span class="p">]</span>
</span><span id="L-50"><a href="#L-50"><span class="linenos"> 50</span></a>
</span><span id="L-51"><a href="#L-51"><span class="linenos"> 51</span></a><span class="n">CredentialProviderFunctionReturn</span> <span class="o">=</span> <span class="nb">tuple</span><span class="p">[</span><span class="nb">dict</span><span class="p">[</span><span class="nb">str</span><span class="p">,</span> <span class="nb">str</span><span class="p">],</span> <span class="n">Union</span><span class="p">[</span><span class="nb">int</span><span class="p">,</span> <span class="kc">None</span><span class="p">]]</span>
</span><span id="L-52"><a href="#L-52"><span class="linenos"> 52</span></a><span class="sd">"""The ``(credentials, expiry)`` pair a credential provider returns."""</span>
</span><span id="L-53"><a href="#L-53"><span class="linenos"> 53</span></a>
</span><span id="L-54"><a href="#L-54"><span class="linenos"> 54</span></a><span class="n">CredentialProviderFunction</span> <span class="o">=</span> <span class="n">Union</span><span class="p">[</span>
</span><span id="L-55"><a href="#L-55"><span class="linenos"> 55</span></a>    <span class="n">Callable</span><span class="p">[[],</span> <span class="n">CredentialProviderFunctionReturn</span><span class="p">],</span> <span class="s2">"CredentialProvider"</span>
</span><span id="L-56"><a href="#L-56"><span class="linenos"> 56</span></a><span class="p">]</span>
</span><span id="L-57"><a href="#L-57"><span class="linenos"> 57</span></a><span class="sd">"""Anything accepted as ``credential_provider``."""</span>
</span><span id="L-58"><a href="#L-58"><span class="linenos"> 58</span></a>
</span><span id="L-59"><a href="#L-59"><span class="linenos"> 59</span></a><span class="c1"># Fetch new credentials this long before the cached ones expire. Longer than</span>
</span><span id="L-60"><a href="#L-60"><span class="linenos"> 60</span></a><span class="c1"># the margin the Rust side uses (60 s), so credentials handed over are always</span>
</span><span id="L-61"><a href="#L-61"><span class="linenos"> 61</span></a><span class="c1"># valid for at least that long.</span>
</span><span id="L-62"><a href="#L-62"><span class="linenos"> 62</span></a><span class="n">_REFRESH_MARGIN_SECONDS</span> <span class="o">=</span> <span class="mi">5</span> <span class="o">*</span> <span class="mi">60</span>
</span><span id="L-63"><a href="#L-63"><span class="linenos"> 63</span></a>
</span><span id="L-64"><a href="#L-64"><span class="linenos"> 64</span></a>
</span><span id="L-65"><a href="#L-65"><span class="linenos"> 65</span></a><span class="k">class</span><span class="w"> </span><span class="nc">AWSAssumeRoleKWArgs</span><span class="p">(</span><span class="n">TypedDict</span><span class="p">,</span> <span class="n">total</span><span class="o">=</span><span class="kc">False</span><span class="p">):</span>
</span><span id="L-66"><a href="#L-66"><span class="linenos"> 66</span></a><span class="w">    </span><span class="sd">"""Parameters for `STS.Client.assume_role()`.</span>
</span><span id="L-67"><a href="#L-67"><span class="linenos"> 67</span></a>
</span><span id="L-68"><a href="#L-68"><span class="linenos"> 68</span></a><span class="sd">    See https://boto3.amazonaws.com/v1/documentation/api/latest/reference/services/sts/client/assume_role.html</span>
</span><span id="L-69"><a href="#L-69"><span class="linenos"> 69</span></a><span class="sd">    """</span>
</span><span id="L-70"><a href="#L-70"><span class="linenos"> 70</span></a>
</span><span id="L-71"><a href="#L-71"><span class="linenos"> 71</span></a>    <span class="n">RoleArn</span><span class="p">:</span> <span class="nb">str</span>
</span><span id="L-72"><a href="#L-72"><span class="linenos"> 72</span></a>    <span class="n">RoleSessionName</span><span class="p">:</span> <span class="nb">str</span>
</span><span id="L-73"><a href="#L-73"><span class="linenos"> 73</span></a>    <span class="n">PolicyArns</span><span class="p">:</span> <span class="nb">list</span><span class="p">[</span><span class="nb">dict</span><span class="p">[</span><span class="nb">str</span><span class="p">,</span> <span class="nb">str</span><span class="p">]]</span>
</span><span id="L-74"><a href="#L-74"><span class="linenos"> 74</span></a>    <span class="n">Policy</span><span class="p">:</span> <span class="nb">str</span>
</span><span id="L-75"><a href="#L-75"><span class="linenos"> 75</span></a>    <span class="n">DurationSeconds</span><span class="p">:</span> <span class="nb">int</span>
</span><span id="L-76"><a href="#L-76"><span class="linenos"> 76</span></a>    <span class="n">Tags</span><span class="p">:</span> <span class="nb">list</span><span class="p">[</span><span class="nb">dict</span><span class="p">[</span><span class="nb">str</span><span class="p">,</span> <span class="nb">str</span><span class="p">]]</span>
</span><span id="L-77"><a href="#L-77"><span class="linenos"> 77</span></a>    <span class="n">TransitiveTagKeys</span><span class="p">:</span> <span class="nb">list</span><span class="p">[</span><span class="nb">str</span><span class="p">]</span>
</span><span id="L-78"><a href="#L-78"><span class="linenos"> 78</span></a>    <span class="n">ExternalId</span><span class="p">:</span> <span class="nb">str</span>
</span><span id="L-79"><a href="#L-79"><span class="linenos"> 79</span></a>    <span class="n">SerialNumber</span><span class="p">:</span> <span class="nb">str</span>
</span><span id="L-80"><a href="#L-80"><span class="linenos"> 80</span></a>    <span class="n">TokenCode</span><span class="p">:</span> <span class="nb">str</span>
</span><span id="L-81"><a href="#L-81"><span class="linenos"> 81</span></a>    <span class="n">SourceIdentity</span><span class="p">:</span> <span class="nb">str</span>
</span><span id="L-82"><a href="#L-82"><span class="linenos"> 82</span></a>    <span class="n">ProvidedContexts</span><span class="p">:</span> <span class="nb">list</span><span class="p">[</span><span class="nb">dict</span><span class="p">[</span><span class="nb">str</span><span class="p">,</span> <span class="nb">str</span><span class="p">]]</span>
</span><span id="L-83"><a href="#L-83"><span class="linenos"> 83</span></a>
</span><span id="L-84"><a href="#L-84"><span class="linenos"> 84</span></a>
</span><span id="L-85"><a href="#L-85"><span class="linenos"> 85</span></a><span class="k">class</span><span class="w"> </span><span class="nc">CredentialProvider</span><span class="p">(</span><span class="n">abc</span><span class="o">.</span><span class="n">ABC</span><span class="p">):</span>
</span><span id="L-86"><a href="#L-86"><span class="linenos"> 86</span></a><span class="w">    </span><span class="sd">"""Base class for credential providers.</span>
</span><span id="L-87"><a href="#L-87"><span class="linenos"> 87</span></a>
</span><span id="L-88"><a href="#L-88"><span class="linenos"> 88</span></a><span class="sd">    Subclasses implement ``retrieve_credentials_impl``. Results are cached and</span>
</span><span id="L-89"><a href="#L-89"><span class="linenos"> 89</span></a><span class="sd">    reused until five minutes before they expire.</span>
</span><span id="L-90"><a href="#L-90"><span class="linenos"> 90</span></a><span class="sd">    """</span>
</span><span id="L-91"><a href="#L-91"><span class="linenos"> 91</span></a>
</span><span id="L-92"><a href="#L-92"><span class="linenos"> 92</span></a>    <span class="k">def</span><span class="w"> </span><span class="fm">__init__</span><span class="p">(</span><span class="bp">self</span><span class="p">)</span> <span class="o">-&gt;</span> <span class="kc">None</span><span class="p">:</span>
</span><span id="L-93"><a href="#L-93"><span class="linenos"> 93</span></a>        <span class="bp">self</span><span class="o">.</span><span class="n">_cached</span><span class="p">:</span> <span class="n">CredentialProviderFunctionReturn</span> <span class="o">|</span> <span class="kc">None</span> <span class="o">=</span> <span class="kc">None</span>
</span><span id="L-94"><a href="#L-94"><span class="linenos"> 94</span></a>        <span class="bp">self</span><span class="o">.</span><span class="n">_lock</span> <span class="o">=</span> <span class="n">threading</span><span class="o">.</span><span class="n">Lock</span><span class="p">()</span>
</span><span id="L-95"><a href="#L-95"><span class="linenos"> 95</span></a>
</span><span id="L-96"><a href="#L-96"><span class="linenos"> 96</span></a>    <span class="k">def</span><span class="w"> </span><span class="fm">__call__</span><span class="p">(</span><span class="bp">self</span><span class="p">)</span> <span class="o">-&gt;</span> <span class="n">CredentialProviderFunctionReturn</span><span class="p">:</span>
</span><span id="L-97"><a href="#L-97"><span class="linenos"> 97</span></a>        <span class="k">with</span> <span class="bp">self</span><span class="o">.</span><span class="n">_lock</span><span class="p">:</span>
</span><span id="L-98"><a href="#L-98"><span class="linenos"> 98</span></a>            <span class="n">cached</span> <span class="o">=</span> <span class="bp">self</span><span class="o">.</span><span class="n">_cached</span>
</span><span id="L-99"><a href="#L-99"><span class="linenos"> 99</span></a>            <span class="k">if</span> <span class="n">cached</span> <span class="ow">is</span> <span class="kc">None</span> <span class="ow">or</span> <span class="p">(</span>
</span><span id="L-100"><a href="#L-100"><span class="linenos">100</span></a>                <span class="n">cached</span><span class="p">[</span><span class="mi">1</span><span class="p">]</span> <span class="ow">is</span> <span class="ow">not</span> <span class="kc">None</span>
</span><span id="L-101"><a href="#L-101"><span class="linenos">101</span></a>                <span class="ow">and</span> <span class="n">cached</span><span class="p">[</span><span class="mi">1</span><span class="p">]</span> <span class="o">-</span> <span class="n">_REFRESH_MARGIN_SECONDS</span> <span class="o">&lt;=</span> <span class="nb">int</span><span class="p">(</span><span class="n">time</span><span class="o">.</span><span class="n">time</span><span class="p">())</span>
</span><span id="L-102"><a href="#L-102"><span class="linenos">102</span></a>            <span class="p">):</span>
</span><span id="L-103"><a href="#L-103"><span class="linenos">103</span></a>                <span class="n">cached</span> <span class="o">=</span> <span class="bp">self</span><span class="o">.</span><span class="n">retrieve_credentials_impl</span><span class="p">()</span>
</span><span id="L-104"><a href="#L-104"><span class="linenos">104</span></a>                <span class="bp">self</span><span class="o">.</span><span class="n">_cached</span> <span class="o">=</span> <span class="n">cached</span>
</span><span id="L-105"><a href="#L-105"><span class="linenos">105</span></a>        <span class="n">credentials</span><span class="p">,</span> <span class="n">expiry</span> <span class="o">=</span> <span class="n">cached</span>
</span><span id="L-106"><a href="#L-106"><span class="linenos">106</span></a>        <span class="k">return</span> <span class="p">{</span><span class="o">**</span><span class="n">credentials</span><span class="p">},</span> <span class="n">expiry</span>
</span><span id="L-107"><a href="#L-107"><span class="linenos">107</span></a>
</span><span id="L-108"><a href="#L-108"><span class="linenos">108</span></a>    <span class="nd">@abc</span><span class="o">.</span><span class="n">abstractmethod</span>
</span><span id="L-109"><a href="#L-109"><span class="linenos">109</span></a>    <span class="k">def</span><span class="w"> </span><span class="nf">retrieve_credentials_impl</span><span class="p">(</span><span class="bp">self</span><span class="p">)</span> <span class="o">-&gt;</span> <span class="n">CredentialProviderFunctionReturn</span><span class="p">:</span>
</span><span id="L-110"><a href="#L-110"><span class="linenos">110</span></a><span class="w">        </span><span class="sd">"""Fetch new credentials, bypassing the cache."""</span>
</span><span id="L-111"><a href="#L-111"><span class="linenos">111</span></a>
</span><span id="L-112"><a href="#L-112"><span class="linenos">112</span></a>    <span class="k">def</span><span class="w"> </span><span class="nf">clear_cached_credentials</span><span class="p">(</span><span class="bp">self</span><span class="p">)</span> <span class="o">-&gt;</span> <span class="kc">None</span><span class="p">:</span>
</span><span id="L-113"><a href="#L-113"><span class="linenos">113</span></a><span class="w">        </span><span class="sd">"""Forget cached credentials, so the next call fetches new ones."""</span>
</span><span id="L-114"><a href="#L-114"><span class="linenos">114</span></a>        <span class="k">with</span> <span class="bp">self</span><span class="o">.</span><span class="n">_lock</span><span class="p">:</span>
</span><span id="L-115"><a href="#L-115"><span class="linenos">115</span></a>            <span class="bp">self</span><span class="o">.</span><span class="n">_cached</span> <span class="o">=</span> <span class="kc">None</span>
</span><span id="L-116"><a href="#L-116"><span class="linenos">116</span></a>
</span><span id="L-117"><a href="#L-117"><span class="linenos">117</span></a>    <span class="k">def</span><span class="w"> </span><span class="nf">__getstate__</span><span class="p">(</span><span class="bp">self</span><span class="p">)</span> <span class="o">-&gt;</span> <span class="nb">dict</span><span class="p">[</span><span class="nb">str</span><span class="p">,</span> <span class="n">Any</span><span class="p">]:</span>
</span><span id="L-118"><a href="#L-118"><span class="linenos">118</span></a>        <span class="c1"># Credentials are secrets: never pickle them. Locks can't be pickled.</span>
</span><span id="L-119"><a href="#L-119"><span class="linenos">119</span></a>        <span class="n">state</span> <span class="o">=</span> <span class="bp">self</span><span class="o">.</span><span class="vm">__dict__</span><span class="o">.</span><span class="n">copy</span><span class="p">()</span>
</span><span id="L-120"><a href="#L-120"><span class="linenos">120</span></a>        <span class="n">state</span><span class="p">[</span><span class="s2">"_cached"</span><span class="p">]</span> <span class="o">=</span> <span class="kc">None</span>
</span><span id="L-121"><a href="#L-121"><span class="linenos">121</span></a>        <span class="k">del</span> <span class="n">state</span><span class="p">[</span><span class="s2">"_lock"</span><span class="p">]</span>
</span><span id="L-122"><a href="#L-122"><span class="linenos">122</span></a>        <span class="k">return</span> <span class="n">state</span>
</span><span id="L-123"><a href="#L-123"><span class="linenos">123</span></a>
</span><span id="L-124"><a href="#L-124"><span class="linenos">124</span></a>    <span class="k">def</span><span class="w"> </span><span class="nf">__setstate__</span><span class="p">(</span><span class="bp">self</span><span class="p">,</span> <span class="n">state</span><span class="p">:</span> <span class="nb">dict</span><span class="p">[</span><span class="nb">str</span><span class="p">,</span> <span class="n">Any</span><span class="p">])</span> <span class="o">-&gt;</span> <span class="kc">None</span><span class="p">:</span>
</span><span id="L-125"><a href="#L-125"><span class="linenos">125</span></a>        <span class="bp">self</span><span class="o">.</span><span class="vm">__dict__</span><span class="o">.</span><span class="n">update</span><span class="p">(</span><span class="n">state</span><span class="p">)</span>
</span><span id="L-126"><a href="#L-126"><span class="linenos">126</span></a>        <span class="bp">self</span><span class="o">.</span><span class="n">_lock</span> <span class="o">=</span> <span class="n">threading</span><span class="o">.</span><span class="n">Lock</span><span class="p">()</span>
</span><span id="L-127"><a href="#L-127"><span class="linenos">127</span></a>
</span><span id="L-128"><a href="#L-128"><span class="linenos">128</span></a>    <span class="k">def</span><span class="w"> </span><span class="fm">__repr__</span><span class="p">(</span><span class="bp">self</span><span class="p">)</span> <span class="o">-&gt;</span> <span class="nb">str</span><span class="p">:</span>
</span><span id="L-129"><a href="#L-129"><span class="linenos">129</span></a>        <span class="k">return</span> <span class="sa">f</span><span class="s2">"</span><span class="si">{</span><span class="nb">type</span><span class="p">(</span><span class="bp">self</span><span class="p">)</span><span class="o">.</span><span class="vm">__name__</span><span class="si">}</span><span class="s2">()"</span>
</span><span id="L-130"><a href="#L-130"><span class="linenos">130</span></a>
</span><span id="L-131"><a href="#L-131"><span class="linenos">131</span></a>
</span><span id="L-132"><a href="#L-132"><span class="linenos">132</span></a><span class="k">def</span><span class="w"> </span><span class="nf">_require</span><span class="p">(</span><span class="n">module</span><span class="p">:</span> <span class="nb">str</span><span class="p">,</span> <span class="n">package</span><span class="p">:</span> <span class="nb">str</span><span class="p">,</span> <span class="n">provider</span><span class="p">:</span> <span class="nb">str</span><span class="p">)</span> <span class="o">-&gt;</span> <span class="kc">None</span><span class="p">:</span>
</span><span id="L-133"><a href="#L-133"><span class="linenos">133</span></a>    <span class="k">if</span> <span class="n">importlib</span><span class="o">.</span><span class="n">util</span><span class="o">.</span><span class="n">find_spec</span><span class="p">(</span><span class="n">module</span><span class="p">)</span> <span class="ow">is</span> <span class="kc">None</span><span class="p">:</span>
</span><span id="L-134"><a href="#L-134"><span class="linenos">134</span></a>        <span class="k">raise</span> <span class="ne">ImportError</span><span class="p">(</span><span class="sa">f</span><span class="s2">"</span><span class="si">{</span><span class="n">package</span><span class="si">}</span><span class="s2"> must be installed to use </span><span class="si">{</span><span class="n">provider</span><span class="si">}</span><span class="s2">"</span><span class="p">)</span>
</span><span id="L-135"><a href="#L-135"><span class="linenos">135</span></a>
</span><span id="L-136"><a href="#L-136"><span class="linenos">136</span></a>
</span><span id="L-137"><a href="#L-137"><span class="linenos">137</span></a><span class="k">class</span><span class="w"> </span><span class="nc">CredentialProviderAWS</span><span class="p">(</span><span class="n">CredentialProvider</span><span class="p">):</span>
</span><span id="L-138"><a href="#L-138"><span class="linenos">138</span></a><span class="w">    </span><span class="sd">"""AWS credentials from boto3: profiles, SSO, assume-role, instance roles.</span>
</span><span id="L-139"><a href="#L-139"><span class="linenos">139</span></a>
</span><span id="L-140"><a href="#L-140"><span class="linenos">140</span></a><span class="sd">    Uses boto3's credential chain, so everything the AWS CLI supports works,</span>
</span><span id="L-141"><a href="#L-141"><span class="linenos">141</span></a><span class="sd">    including ``AWS_PROFILE`` and SSO sessions. Requires ``boto3``.</span>
</span><span id="L-142"><a href="#L-142"><span class="linenos">142</span></a>
</span><span id="L-143"><a href="#L-143"><span class="linenos">143</span></a><span class="sd">    Args:</span>
</span><span id="L-144"><a href="#L-144"><span class="linenos">144</span></a><span class="sd">        profile_name: Profile to use from ``~/.aws/config``.</span>
</span><span id="L-145"><a href="#L-145"><span class="linenos">145</span></a><span class="sd">        region_name: Region to use; defaults to the profile's region.</span>
</span><span id="L-146"><a href="#L-146"><span class="linenos">146</span></a><span class="sd">        assume_role: If given, assume this role with the session's</span>
</span><span id="L-147"><a href="#L-147"><span class="linenos">147</span></a><span class="sd">            credentials. The keys are passed to ``STS.Client.assume_role()``,</span>
</span><span id="L-148"><a href="#L-148"><span class="linenos">148</span></a><span class="sd">            e.g. ``{"RoleArn": "...", "RoleSessionName": "tantivy"}``.</span>
</span><span id="L-149"><a href="#L-149"><span class="linenos">149</span></a><span class="sd">    """</span>
</span><span id="L-150"><a href="#L-150"><span class="linenos">150</span></a>
</span><span id="L-151"><a href="#L-151"><span class="linenos">151</span></a>    <span class="k">def</span><span class="w"> </span><span class="fm">__init__</span><span class="p">(</span>
</span><span id="L-152"><a href="#L-152"><span class="linenos">152</span></a>        <span class="bp">self</span><span class="p">,</span>
</span><span id="L-153"><a href="#L-153"><span class="linenos">153</span></a>        <span class="o">*</span><span class="p">,</span>
</span><span id="L-154"><a href="#L-154"><span class="linenos">154</span></a>        <span class="n">profile_name</span><span class="p">:</span> <span class="nb">str</span> <span class="o">|</span> <span class="kc">None</span> <span class="o">=</span> <span class="kc">None</span><span class="p">,</span>
</span><span id="L-155"><a href="#L-155"><span class="linenos">155</span></a>        <span class="n">region_name</span><span class="p">:</span> <span class="nb">str</span> <span class="o">|</span> <span class="kc">None</span> <span class="o">=</span> <span class="kc">None</span><span class="p">,</span>
</span><span id="L-156"><a href="#L-156"><span class="linenos">156</span></a>        <span class="n">assume_role</span><span class="p">:</span> <span class="n">AWSAssumeRoleKWArgs</span> <span class="o">|</span> <span class="kc">None</span> <span class="o">=</span> <span class="kc">None</span><span class="p">,</span>
</span><span id="L-157"><a href="#L-157"><span class="linenos">157</span></a>    <span class="p">)</span> <span class="o">-&gt;</span> <span class="kc">None</span><span class="p">:</span>
</span><span id="L-158"><a href="#L-158"><span class="linenos">158</span></a>        <span class="n">_require</span><span class="p">(</span><span class="s2">"boto3"</span><span class="p">,</span> <span class="s2">"boto3"</span><span class="p">,</span> <span class="s2">"CredentialProviderAWS"</span><span class="p">)</span>
</span><span id="L-159"><a href="#L-159"><span class="linenos">159</span></a>        <span class="bp">self</span><span class="o">.</span><span class="n">profile_name</span> <span class="o">=</span> <span class="n">profile_name</span>
</span><span id="L-160"><a href="#L-160"><span class="linenos">160</span></a>        <span class="bp">self</span><span class="o">.</span><span class="n">region_name</span> <span class="o">=</span> <span class="n">region_name</span>
</span><span id="L-161"><a href="#L-161"><span class="linenos">161</span></a>        <span class="bp">self</span><span class="o">.</span><span class="n">assume_role</span> <span class="o">=</span> <span class="n">assume_role</span>
</span><span id="L-162"><a href="#L-162"><span class="linenos">162</span></a>        <span class="nb">super</span><span class="p">()</span><span class="o">.</span><span class="fm">__init__</span><span class="p">()</span>
</span><span id="L-163"><a href="#L-163"><span class="linenos">163</span></a>
</span><span id="L-164"><a href="#L-164"><span class="linenos">164</span></a>    <span class="k">def</span><span class="w"> </span><span class="nf">retrieve_credentials_impl</span><span class="p">(</span><span class="bp">self</span><span class="p">)</span> <span class="o">-&gt;</span> <span class="n">CredentialProviderFunctionReturn</span><span class="p">:</span>
</span><span id="L-165"><a href="#L-165"><span class="linenos">165</span></a>        <span class="n">session</span> <span class="o">=</span> <span class="bp">self</span><span class="o">.</span><span class="n">_session</span><span class="p">()</span>
</span><span id="L-166"><a href="#L-166"><span class="linenos">166</span></a>        <span class="k">if</span> <span class="bp">self</span><span class="o">.</span><span class="n">assume_role</span> <span class="ow">is</span> <span class="ow">not</span> <span class="kc">None</span><span class="p">:</span>
</span><span id="L-167"><a href="#L-167"><span class="linenos">167</span></a>            <span class="k">return</span> <span class="bp">self</span><span class="o">.</span><span class="n">_assume_role</span><span class="p">(</span><span class="n">session</span><span class="p">)</span>
</span><span id="L-168"><a href="#L-168"><span class="linenos">168</span></a>
</span><span id="L-169"><a href="#L-169"><span class="linenos">169</span></a>        <span class="n">creds</span> <span class="o">=</span> <span class="n">session</span><span class="o">.</span><span class="n">get_credentials</span><span class="p">()</span>
</span><span id="L-170"><a href="#L-170"><span class="linenos">170</span></a>        <span class="k">if</span> <span class="n">creds</span> <span class="ow">is</span> <span class="kc">None</span><span class="p">:</span>
</span><span id="L-171"><a href="#L-171"><span class="linenos">171</span></a>            <span class="k">raise</span> <span class="ne">RuntimeError</span><span class="p">(</span>
</span><span id="L-172"><a href="#L-172"><span class="linenos">172</span></a>                <span class="s2">"boto3 found no AWS credentials; configure a profile, "</span>
</span><span id="L-173"><a href="#L-173"><span class="linenos">173</span></a>                <span class="s2">"environment variables or an instance role"</span>
</span><span id="L-174"><a href="#L-174"><span class="linenos">174</span></a>            <span class="p">)</span>
</span><span id="L-175"><a href="#L-175"><span class="linenos">175</span></a>        <span class="c1"># Read the keys before the expiry: for refreshable credentials,</span>
</span><span id="L-176"><a href="#L-176"><span class="linenos">176</span></a>        <span class="c1"># reading them is what triggers the refresh that sets the expiry.</span>
</span><span id="L-177"><a href="#L-177"><span class="linenos">177</span></a>        <span class="n">frozen</span> <span class="o">=</span> <span class="n">creds</span><span class="o">.</span><span class="n">get_frozen_credentials</span><span class="p">()</span>
</span><span id="L-178"><a href="#L-178"><span class="linenos">178</span></a>        <span class="n">credentials</span> <span class="o">=</span> <span class="p">{</span>
</span><span id="L-179"><a href="#L-179"><span class="linenos">179</span></a>            <span class="s2">"aws_access_key_id"</span><span class="p">:</span> <span class="n">frozen</span><span class="o">.</span><span class="n">access_key</span><span class="p">,</span>
</span><span id="L-180"><a href="#L-180"><span class="linenos">180</span></a>            <span class="s2">"aws_secret_access_key"</span><span class="p">:</span> <span class="n">frozen</span><span class="o">.</span><span class="n">secret_key</span><span class="p">,</span>
</span><span id="L-181"><a href="#L-181"><span class="linenos">181</span></a>        <span class="p">}</span>
</span><span id="L-182"><a href="#L-182"><span class="linenos">182</span></a>        <span class="k">if</span> <span class="n">frozen</span><span class="o">.</span><span class="n">token</span> <span class="ow">is</span> <span class="ow">not</span> <span class="kc">None</span><span class="p">:</span>
</span><span id="L-183"><a href="#L-183"><span class="linenos">183</span></a>            <span class="n">credentials</span><span class="p">[</span><span class="s2">"aws_session_token"</span><span class="p">]</span> <span class="o">=</span> <span class="n">frozen</span><span class="o">.</span><span class="n">token</span>
</span><span id="L-184"><a href="#L-184"><span class="linenos">184</span></a>        <span class="n">expiry</span> <span class="o">=</span> <span class="nb">getattr</span><span class="p">(</span><span class="n">creds</span><span class="p">,</span> <span class="s2">"_expiry_time"</span><span class="p">,</span> <span class="kc">None</span><span class="p">)</span>
</span><span id="L-185"><a href="#L-185"><span class="linenos">185</span></a>        <span class="k">return</span> <span class="n">credentials</span><span class="p">,</span> <span class="p">(</span>
</span><span id="L-186"><a href="#L-186"><span class="linenos">186</span></a>            <span class="nb">int</span><span class="p">(</span><span class="n">expiry</span><span class="o">.</span><span class="n">timestamp</span><span class="p">())</span> <span class="k">if</span> <span class="nb">isinstance</span><span class="p">(</span><span class="n">expiry</span><span class="p">,</span> <span class="n">datetime</span><span class="p">)</span> <span class="k">else</span> <span class="kc">None</span>
</span><span id="L-187"><a href="#L-187"><span class="linenos">187</span></a>        <span class="p">)</span>
</span><span id="L-188"><a href="#L-188"><span class="linenos">188</span></a>
</span><span id="L-189"><a href="#L-189"><span class="linenos">189</span></a>    <span class="k">def</span><span class="w"> </span><span class="nf">_assume_role</span><span class="p">(</span><span class="bp">self</span><span class="p">,</span> <span class="n">session</span><span class="p">:</span> <span class="n">Any</span><span class="p">)</span> <span class="o">-&gt;</span> <span class="n">CredentialProviderFunctionReturn</span><span class="p">:</span>
</span><span id="L-190"><a href="#L-190"><span class="linenos">190</span></a>        <span class="k">assert</span> <span class="bp">self</span><span class="o">.</span><span class="n">assume_role</span> <span class="ow">is</span> <span class="ow">not</span> <span class="kc">None</span>
</span><span id="L-191"><a href="#L-191"><span class="linenos">191</span></a>        <span class="n">response</span> <span class="o">=</span> <span class="n">session</span><span class="o">.</span><span class="n">client</span><span class="p">(</span><span class="s2">"sts"</span><span class="p">)</span><span class="o">.</span><span class="n">assume_role</span><span class="p">(</span><span class="o">**</span><span class="bp">self</span><span class="o">.</span><span class="n">assume_role</span><span class="p">)</span>
</span><span id="L-192"><a href="#L-192"><span class="linenos">192</span></a>        <span class="n">creds</span> <span class="o">=</span> <span class="n">response</span><span class="p">[</span><span class="s2">"Credentials"</span><span class="p">]</span>
</span><span id="L-193"><a href="#L-193"><span class="linenos">193</span></a>        <span class="n">expiry</span> <span class="o">=</span> <span class="n">creds</span><span class="p">[</span><span class="s2">"Expiration"</span><span class="p">]</span>
</span><span id="L-194"><a href="#L-194"><span class="linenos">194</span></a>        <span class="k">if</span> <span class="n">expiry</span><span class="o">.</span><span class="n">tzinfo</span> <span class="ow">is</span> <span class="kc">None</span><span class="p">:</span>
</span><span id="L-195"><a href="#L-195"><span class="linenos">195</span></a>            <span class="k">raise</span> <span class="ne">ValueError</span><span class="p">(</span><span class="s2">"STS returned an expiration time with no time zone"</span><span class="p">)</span>
</span><span id="L-196"><a href="#L-196"><span class="linenos">196</span></a>        <span class="k">return</span> <span class="p">{</span>
</span><span id="L-197"><a href="#L-197"><span class="linenos">197</span></a>            <span class="s2">"aws_access_key_id"</span><span class="p">:</span> <span class="n">creds</span><span class="p">[</span><span class="s2">"AccessKeyId"</span><span class="p">],</span>
</span><span id="L-198"><a href="#L-198"><span class="linenos">198</span></a>            <span class="s2">"aws_secret_access_key"</span><span class="p">:</span> <span class="n">creds</span><span class="p">[</span><span class="s2">"SecretAccessKey"</span><span class="p">],</span>
</span><span id="L-199"><a href="#L-199"><span class="linenos">199</span></a>            <span class="s2">"aws_session_token"</span><span class="p">:</span> <span class="n">creds</span><span class="p">[</span><span class="s2">"SessionToken"</span><span class="p">],</span>
</span><span id="L-200"><a href="#L-200"><span class="linenos">200</span></a>        <span class="p">},</span> <span class="nb">int</span><span class="p">(</span><span class="n">expiry</span><span class="o">.</span><span class="n">timestamp</span><span class="p">())</span>
</span><span id="L-201"><a href="#L-201"><span class="linenos">201</span></a>
</span><span id="L-202"><a href="#L-202"><span class="linenos">202</span></a>    <span class="k">def</span><span class="w"> </span><span class="nf">_storage_options</span><span class="p">(</span><span class="bp">self</span><span class="p">)</span> <span class="o">-&gt;</span> <span class="nb">dict</span><span class="p">[</span><span class="nb">str</span><span class="p">,</span> <span class="nb">str</span><span class="p">]:</span>
</span><span id="L-203"><a href="#L-203"><span class="linenos">203</span></a><span class="w">        </span><span class="sd">"""Settings tantivy uses unless storage_options gives them.</span>
</span><span id="L-204"><a href="#L-204"><span class="linenos">204</span></a>
</span><span id="L-205"><a href="#L-205"><span class="linenos">205</span></a><span class="sd">        Called from Rust when the store is built: passes on the profile's</span>
</span><span id="L-206"><a href="#L-206"><span class="linenos">206</span></a><span class="sd">        region, which object_store cannot read for a named profile.</span>
</span><span id="L-207"><a href="#L-207"><span class="linenos">207</span></a><span class="sd">        """</span>
</span><span id="L-208"><a href="#L-208"><span class="linenos">208</span></a>        <span class="n">region</span> <span class="o">=</span> <span class="bp">self</span><span class="o">.</span><span class="n">region_name</span> <span class="ow">or</span> <span class="bp">self</span><span class="o">.</span><span class="n">_session</span><span class="p">()</span><span class="o">.</span><span class="n">region_name</span>
</span><span id="L-209"><a href="#L-209"><span class="linenos">209</span></a>        <span class="k">return</span> <span class="p">{</span><span class="s2">"region"</span><span class="p">:</span> <span class="n">region</span><span class="p">}</span> <span class="k">if</span> <span class="n">region</span> <span class="k">else</span> <span class="p">{}</span>
</span><span id="L-210"><a href="#L-210"><span class="linenos">210</span></a>
</span><span id="L-211"><a href="#L-211"><span class="linenos">211</span></a>    <span class="k">def</span><span class="w"> </span><span class="nf">_session</span><span class="p">(</span><span class="bp">self</span><span class="p">)</span> <span class="o">-&gt;</span> <span class="n">Any</span><span class="p">:</span>
</span><span id="L-212"><a href="#L-212"><span class="linenos">212</span></a>        <span class="kn">import</span><span class="w"> </span><span class="nn">boto3</span>
</span><span id="L-213"><a href="#L-213"><span class="linenos">213</span></a>
</span><span id="L-214"><a href="#L-214"><span class="linenos">214</span></a>        <span class="c1"># boto3 reads AWS_PROFILE itself when profile_name is None.</span>
</span><span id="L-215"><a href="#L-215"><span class="linenos">215</span></a>        <span class="k">return</span> <span class="n">boto3</span><span class="o">.</span><span class="n">Session</span><span class="p">(</span>
</span><span id="L-216"><a href="#L-216"><span class="linenos">216</span></a>            <span class="n">profile_name</span><span class="o">=</span><span class="bp">self</span><span class="o">.</span><span class="n">profile_name</span><span class="p">,</span> <span class="n">region_name</span><span class="o">=</span><span class="bp">self</span><span class="o">.</span><span class="n">region_name</span>
</span><span id="L-217"><a href="#L-217"><span class="linenos">217</span></a>        <span class="p">)</span>
</span><span id="L-218"><a href="#L-218"><span class="linenos">218</span></a>
</span><span id="L-219"><a href="#L-219"><span class="linenos">219</span></a>    <span class="k">def</span><span class="w"> </span><span class="fm">__repr__</span><span class="p">(</span><span class="bp">self</span><span class="p">)</span> <span class="o">-&gt;</span> <span class="nb">str</span><span class="p">:</span>
</span><span id="L-220"><a href="#L-220"><span class="linenos">220</span></a>        <span class="k">return</span> <span class="sa">f</span><span class="s2">"CredentialProviderAWS(profile_name=</span><span class="si">{</span><span class="bp">self</span><span class="o">.</span><span class="n">profile_name</span><span class="si">!r}</span><span class="s2">)"</span>
</span><span id="L-221"><a href="#L-221"><span class="linenos">221</span></a>
</span><span id="L-222"><a href="#L-222"><span class="linenos">222</span></a>
</span><span id="L-223"><a href="#L-223"><span class="linenos">223</span></a><span class="k">class</span><span class="w"> </span><span class="nc">CredentialProviderGCP</span><span class="p">(</span><span class="n">CredentialProvider</span><span class="p">):</span>
</span><span id="L-224"><a href="#L-224"><span class="linenos">224</span></a><span class="w">    </span><span class="sd">"""Google Cloud credentials from ``google.auth.default()``.</span>
</span><span id="L-225"><a href="#L-225"><span class="linenos">225</span></a>
</span><span id="L-226"><a href="#L-226"><span class="linenos">226</span></a><span class="sd">    Covers application default credentials, ``gcloud auth``, service account</span>
</span><span id="L-227"><a href="#L-227"><span class="linenos">227</span></a><span class="sd">    files (``GOOGLE_APPLICATION_CREDENTIALS``) and workload identity.</span>
</span><span id="L-228"><a href="#L-228"><span class="linenos">228</span></a><span class="sd">    Requires ``google-auth``. The arguments are passed to</span>
</span><span id="L-229"><a href="#L-229"><span class="linenos">229</span></a><span class="sd">    ``google.auth.default()``.</span>
</span><span id="L-230"><a href="#L-230"><span class="linenos">230</span></a><span class="sd">    """</span>
</span><span id="L-231"><a href="#L-231"><span class="linenos">231</span></a>
</span><span id="L-232"><a href="#L-232"><span class="linenos">232</span></a>    <span class="k">def</span><span class="w"> </span><span class="fm">__init__</span><span class="p">(</span>
</span><span id="L-233"><a href="#L-233"><span class="linenos">233</span></a>        <span class="bp">self</span><span class="p">,</span>
</span><span id="L-234"><a href="#L-234"><span class="linenos">234</span></a>        <span class="o">*</span><span class="p">,</span>
</span><span id="L-235"><a href="#L-235"><span class="linenos">235</span></a>        <span class="n">scopes</span><span class="p">:</span> <span class="n">Any</span> <span class="o">|</span> <span class="kc">None</span> <span class="o">=</span> <span class="kc">None</span><span class="p">,</span>
</span><span id="L-236"><a href="#L-236"><span class="linenos">236</span></a>        <span class="n">request</span><span class="p">:</span> <span class="n">Any</span> <span class="o">|</span> <span class="kc">None</span> <span class="o">=</span> <span class="kc">None</span><span class="p">,</span>
</span><span id="L-237"><a href="#L-237"><span class="linenos">237</span></a>        <span class="n">quota_project_id</span><span class="p">:</span> <span class="n">Any</span> <span class="o">|</span> <span class="kc">None</span> <span class="o">=</span> <span class="kc">None</span><span class="p">,</span>
</span><span id="L-238"><a href="#L-238"><span class="linenos">238</span></a>        <span class="n">default_scopes</span><span class="p">:</span> <span class="n">Any</span> <span class="o">|</span> <span class="kc">None</span> <span class="o">=</span> <span class="kc">None</span><span class="p">,</span>
</span><span id="L-239"><a href="#L-239"><span class="linenos">239</span></a>    <span class="p">)</span> <span class="o">-&gt;</span> <span class="kc">None</span><span class="p">:</span>
</span><span id="L-240"><a href="#L-240"><span class="linenos">240</span></a>        <span class="n">_require</span><span class="p">(</span><span class="s2">"google.auth"</span><span class="p">,</span> <span class="s2">"google-auth"</span><span class="p">,</span> <span class="s2">"CredentialProviderGCP"</span><span class="p">)</span>
</span><span id="L-241"><a href="#L-241"><span class="linenos">241</span></a>        <span class="bp">self</span><span class="o">.</span><span class="n">scopes</span> <span class="o">=</span> <span class="p">(</span>
</span><span id="L-242"><a href="#L-242"><span class="linenos">242</span></a>            <span class="n">scopes</span>
</span><span id="L-243"><a href="#L-243"><span class="linenos">243</span></a>            <span class="k">if</span> <span class="n">scopes</span> <span class="ow">is</span> <span class="ow">not</span> <span class="kc">None</span>
</span><span id="L-244"><a href="#L-244"><span class="linenos">244</span></a>            <span class="k">else</span> <span class="p">[</span><span class="s2">"https://www.googleapis.com/auth/cloud-platform"</span><span class="p">]</span>
</span><span id="L-245"><a href="#L-245"><span class="linenos">245</span></a>        <span class="p">)</span>
</span><span id="L-246"><a href="#L-246"><span class="linenos">246</span></a>        <span class="bp">self</span><span class="o">.</span><span class="n">request</span> <span class="o">=</span> <span class="n">request</span>
</span><span id="L-247"><a href="#L-247"><span class="linenos">247</span></a>        <span class="bp">self</span><span class="o">.</span><span class="n">quota_project_id</span> <span class="o">=</span> <span class="n">quota_project_id</span>
</span><span id="L-248"><a href="#L-248"><span class="linenos">248</span></a>        <span class="bp">self</span><span class="o">.</span><span class="n">default_scopes</span> <span class="o">=</span> <span class="n">default_scopes</span>
</span><span id="L-249"><a href="#L-249"><span class="linenos">249</span></a>        <span class="nb">super</span><span class="p">()</span><span class="o">.</span><span class="fm">__init__</span><span class="p">()</span>
</span><span id="L-250"><a href="#L-250"><span class="linenos">250</span></a>
</span><span id="L-251"><a href="#L-251"><span class="linenos">251</span></a>    <span class="k">def</span><span class="w"> </span><span class="nf">retrieve_credentials_impl</span><span class="p">(</span><span class="bp">self</span><span class="p">)</span> <span class="o">-&gt;</span> <span class="n">CredentialProviderFunctionReturn</span><span class="p">:</span>
</span><span id="L-252"><a href="#L-252"><span class="linenos">252</span></a>        <span class="kn">import</span><span class="w"> </span><span class="nn">google.auth</span>
</span><span id="L-253"><a href="#L-253"><span class="linenos">253</span></a>        <span class="kn">import</span><span class="w"> </span><span class="nn">google.auth.transport.requests</span>
</span><span id="L-254"><a href="#L-254"><span class="linenos">254</span></a>
</span><span id="L-255"><a href="#L-255"><span class="linenos">255</span></a>        <span class="n">creds</span><span class="p">,</span> <span class="n">_project_id</span> <span class="o">=</span> <span class="n">google</span><span class="o">.</span><span class="n">auth</span><span class="o">.</span><span class="n">default</span><span class="p">(</span>
</span><span id="L-256"><a href="#L-256"><span class="linenos">256</span></a>            <span class="n">scopes</span><span class="o">=</span><span class="bp">self</span><span class="o">.</span><span class="n">scopes</span><span class="p">,</span>
</span><span id="L-257"><a href="#L-257"><span class="linenos">257</span></a>            <span class="n">request</span><span class="o">=</span><span class="bp">self</span><span class="o">.</span><span class="n">request</span><span class="p">,</span>
</span><span id="L-258"><a href="#L-258"><span class="linenos">258</span></a>            <span class="n">quota_project_id</span><span class="o">=</span><span class="bp">self</span><span class="o">.</span><span class="n">quota_project_id</span><span class="p">,</span>
</span><span id="L-259"><a href="#L-259"><span class="linenos">259</span></a>            <span class="n">default_scopes</span><span class="o">=</span><span class="bp">self</span><span class="o">.</span><span class="n">default_scopes</span><span class="p">,</span>
</span><span id="L-260"><a href="#L-260"><span class="linenos">260</span></a>        <span class="p">)</span>
</span><span id="L-261"><a href="#L-261"><span class="linenos">261</span></a>        <span class="n">creds</span><span class="o">.</span><span class="n">refresh</span><span class="p">(</span><span class="n">google</span><span class="o">.</span><span class="n">auth</span><span class="o">.</span><span class="n">transport</span><span class="o">.</span><span class="n">requests</span><span class="o">.</span><span class="n">Request</span><span class="p">())</span>
</span><span id="L-262"><a href="#L-262"><span class="linenos">262</span></a>        <span class="n">expiry</span> <span class="o">=</span> <span class="n">creds</span><span class="o">.</span><span class="n">expiry</span>
</span><span id="L-263"><a href="#L-263"><span class="linenos">263</span></a>        <span class="k">if</span> <span class="n">expiry</span> <span class="ow">is</span> <span class="ow">not</span> <span class="kc">None</span> <span class="ow">and</span> <span class="n">expiry</span><span class="o">.</span><span class="n">tzinfo</span> <span class="ow">is</span> <span class="kc">None</span><span class="p">:</span>
</span><span id="L-264"><a href="#L-264"><span class="linenos">264</span></a>            <span class="c1"># google-auth reports naive UTC times.</span>
</span><span id="L-265"><a href="#L-265"><span class="linenos">265</span></a>            <span class="n">expiry</span> <span class="o">=</span> <span class="n">expiry</span><span class="o">.</span><span class="n">replace</span><span class="p">(</span><span class="n">tzinfo</span><span class="o">=</span><span class="n">timezone</span><span class="o">.</span><span class="n">utc</span><span class="p">)</span>
</span><span id="L-266"><a href="#L-266"><span class="linenos">266</span></a>        <span class="k">return</span> <span class="p">{</span><span class="s2">"bearer_token"</span><span class="p">:</span> <span class="n">creds</span><span class="o">.</span><span class="n">token</span><span class="p">},</span> <span class="p">(</span>
</span><span id="L-267"><a href="#L-267"><span class="linenos">267</span></a>            <span class="nb">int</span><span class="p">(</span><span class="n">expiry</span><span class="o">.</span><span class="n">timestamp</span><span class="p">())</span> <span class="k">if</span> <span class="n">expiry</span> <span class="ow">is</span> <span class="ow">not</span> <span class="kc">None</span> <span class="k">else</span> <span class="kc">None</span>
</span><span id="L-268"><a href="#L-268"><span class="linenos">268</span></a>        <span class="p">)</span>
</span><span id="L-269"><a href="#L-269"><span class="linenos">269</span></a>
</span><span id="L-270"><a href="#L-270"><span class="linenos">270</span></a>
</span><span id="L-271"><a href="#L-271"><span class="linenos">271</span></a><span class="k">class</span><span class="w"> </span><span class="nc">CredentialProviderAzure</span><span class="p">(</span><span class="n">CredentialProvider</span><span class="p">):</span>
</span><span id="L-272"><a href="#L-272"><span class="linenos">272</span></a><span class="w">    </span><span class="sd">"""Azure credentials from ``azure.identity``.</span>
</span><span id="L-273"><a href="#L-273"><span class="linenos">273</span></a>
</span><span id="L-274"><a href="#L-274"><span class="linenos">274</span></a><span class="sd">    By default uses ``DefaultAzureCredential()``, which covers environment</span>
</span><span id="L-275"><a href="#L-275"><span class="linenos">275</span></a><span class="sd">    variables, managed identity and ``az login``. Requires ``azure-identity``</span>
</span><span id="L-276"><a href="#L-276"><span class="linenos">276</span></a><span class="sd">    unless ``credential`` is given.</span>
</span><span id="L-277"><a href="#L-277"><span class="linenos">277</span></a>
</span><span id="L-278"><a href="#L-278"><span class="linenos">278</span></a><span class="sd">    Args:</span>
</span><span id="L-279"><a href="#L-279"><span class="linenos">279</span></a><span class="sd">        scopes: Scopes to request a token for.</span>
</span><span id="L-280"><a href="#L-280"><span class="linenos">280</span></a><span class="sd">        tenant_id: Azure tenant ID.</span>
</span><span id="L-281"><a href="#L-281"><span class="linenos">281</span></a><span class="sd">        credential: An Azure credential object to use instead, such as</span>
</span><span id="L-282"><a href="#L-282"><span class="linenos">282</span></a><span class="sd">            ``azure.identity.ClientSecretCredential(...)``. It must have a</span>
</span><span id="L-283"><a href="#L-283"><span class="linenos">283</span></a><span class="sd">            ``get_token()`` method.</span>
</span><span id="L-284"><a href="#L-284"><span class="linenos">284</span></a><span class="sd">    """</span>
</span><span id="L-285"><a href="#L-285"><span class="linenos">285</span></a>
</span><span id="L-286"><a href="#L-286"><span class="linenos">286</span></a>    <span class="k">def</span><span class="w"> </span><span class="fm">__init__</span><span class="p">(</span>
</span><span id="L-287"><a href="#L-287"><span class="linenos">287</span></a>        <span class="bp">self</span><span class="p">,</span>
</span><span id="L-288"><a href="#L-288"><span class="linenos">288</span></a>        <span class="o">*</span><span class="p">,</span>
</span><span id="L-289"><a href="#L-289"><span class="linenos">289</span></a>        <span class="n">scopes</span><span class="p">:</span> <span class="nb">list</span><span class="p">[</span><span class="nb">str</span><span class="p">]</span> <span class="o">|</span> <span class="kc">None</span> <span class="o">=</span> <span class="kc">None</span><span class="p">,</span>
</span><span id="L-290"><a href="#L-290"><span class="linenos">290</span></a>        <span class="n">tenant_id</span><span class="p">:</span> <span class="nb">str</span> <span class="o">|</span> <span class="kc">None</span> <span class="o">=</span> <span class="kc">None</span><span class="p">,</span>
</span><span id="L-291"><a href="#L-291"><span class="linenos">291</span></a>        <span class="n">credential</span><span class="p">:</span> <span class="n">Any</span> <span class="o">|</span> <span class="kc">None</span> <span class="o">=</span> <span class="kc">None</span><span class="p">,</span>
</span><span id="L-292"><a href="#L-292"><span class="linenos">292</span></a>    <span class="p">)</span> <span class="o">-&gt;</span> <span class="kc">None</span><span class="p">:</span>
</span><span id="L-293"><a href="#L-293"><span class="linenos">293</span></a>        <span class="k">if</span> <span class="n">credential</span> <span class="ow">is</span> <span class="kc">None</span><span class="p">:</span>
</span><span id="L-294"><a href="#L-294"><span class="linenos">294</span></a>            <span class="n">_require</span><span class="p">(</span><span class="s2">"azure.identity"</span><span class="p">,</span> <span class="s2">"azure-identity"</span><span class="p">,</span> <span class="s2">"CredentialProviderAzure"</span><span class="p">)</span>
</span><span id="L-295"><a href="#L-295"><span class="linenos">295</span></a>        <span class="k">elif</span> <span class="ow">not</span> <span class="nb">hasattr</span><span class="p">(</span><span class="n">credential</span><span class="p">,</span> <span class="s2">"get_token"</span><span class="p">):</span>
</span><span id="L-296"><a href="#L-296"><span class="linenos">296</span></a>            <span class="k">raise</span> <span class="ne">ValueError</span><span class="p">(</span>
</span><span id="L-297"><a href="#L-297"><span class="linenos">297</span></a>                <span class="sa">f</span><span class="s2">"credential </span><span class="si">{</span><span class="n">credential</span><span class="si">!r}</span><span class="s2"> does not have a get_token() method"</span>
</span><span id="L-298"><a href="#L-298"><span class="linenos">298</span></a>            <span class="p">)</span>
</span><span id="L-299"><a href="#L-299"><span class="linenos">299</span></a>        <span class="bp">self</span><span class="o">.</span><span class="n">scopes</span> <span class="o">=</span> <span class="p">(</span>
</span><span id="L-300"><a href="#L-300"><span class="linenos">300</span></a>            <span class="n">scopes</span> <span class="k">if</span> <span class="n">scopes</span> <span class="ow">is</span> <span class="ow">not</span> <span class="kc">None</span> <span class="k">else</span> <span class="p">[</span><span class="s2">"https://storage.azure.com/.default"</span><span class="p">]</span>
</span><span id="L-301"><a href="#L-301"><span class="linenos">301</span></a>        <span class="p">)</span>
</span><span id="L-302"><a href="#L-302"><span class="linenos">302</span></a>        <span class="bp">self</span><span class="o">.</span><span class="n">tenant_id</span> <span class="o">=</span> <span class="n">tenant_id</span>
</span><span id="L-303"><a href="#L-303"><span class="linenos">303</span></a>        <span class="bp">self</span><span class="o">.</span><span class="n">credential</span> <span class="o">=</span> <span class="n">credential</span>
</span><span id="L-304"><a href="#L-304"><span class="linenos">304</span></a>        <span class="nb">super</span><span class="p">()</span><span class="o">.</span><span class="fm">__init__</span><span class="p">()</span>
</span><span id="L-305"><a href="#L-305"><span class="linenos">305</span></a>
</span><span id="L-306"><a href="#L-306"><span class="linenos">306</span></a>    <span class="k">def</span><span class="w"> </span><span class="nf">retrieve_credentials_impl</span><span class="p">(</span><span class="bp">self</span><span class="p">)</span> <span class="o">-&gt;</span> <span class="n">CredentialProviderFunctionReturn</span><span class="p">:</span>
</span><span id="L-307"><a href="#L-307"><span class="linenos">307</span></a>        <span class="n">credential</span> <span class="o">=</span> <span class="bp">self</span><span class="o">.</span><span class="n">credential</span>
</span><span id="L-308"><a href="#L-308"><span class="linenos">308</span></a>        <span class="k">if</span> <span class="n">credential</span> <span class="ow">is</span> <span class="kc">None</span><span class="p">:</span>
</span><span id="L-309"><a href="#L-309"><span class="linenos">309</span></a>            <span class="kn">import</span><span class="w"> </span><span class="nn">azure.identity</span>
</span><span id="L-310"><a href="#L-310"><span class="linenos">310</span></a>
</span><span id="L-311"><a href="#L-311"><span class="linenos">311</span></a>            <span class="n">credential</span> <span class="o">=</span> <span class="n">azure</span><span class="o">.</span><span class="n">identity</span><span class="o">.</span><span class="n">DefaultAzureCredential</span><span class="p">()</span>
</span><span id="L-312"><a href="#L-312"><span class="linenos">312</span></a>        <span class="n">token</span> <span class="o">=</span> <span class="n">credential</span><span class="o">.</span><span class="n">get_token</span><span class="p">(</span><span class="o">*</span><span class="bp">self</span><span class="o">.</span><span class="n">scopes</span><span class="p">,</span> <span class="n">tenant_id</span><span class="o">=</span><span class="bp">self</span><span class="o">.</span><span class="n">tenant_id</span><span class="p">)</span>
</span><span id="L-313"><a href="#L-313"><span class="linenos">313</span></a>        <span class="k">return</span> <span class="p">{</span><span class="s2">"bearer_token"</span><span class="p">:</span> <span class="n">token</span><span class="o">.</span><span class="n">token</span><span class="p">},</span> <span class="n">token</span><span class="o">.</span><span class="n">expires_on</span>
</span></pre></div>


            </section>
</main>

## CredentialProvider

<main class="pdoc">
<section id="CredentialProvider">
                            <input id="CredentialProvider-view-source" class="view-source-toggle-state" type="checkbox" aria-hidden="true" tabindex="-1">
<div class="attr class">
            
    <span class="def">class</span>
    <span class="name">CredentialProvider</span><wbr>(<span class="base">abc.ABC</span>):

                <label class="view-source-button" for="CredentialProvider-view-source"><span>View Source</span></label>

    </div>
    <a class="headerlink" href="#CredentialProvider"></a>
            <div class="pdoc-code codehilite"><pre><span></span><span id="CredentialProvider-86"><a href="#CredentialProvider-86"><span class="linenos"> 86</span></a><span class="k">class</span><span class="w"> </span><span class="nc">CredentialProvider</span><span class="p">(</span><span class="n">abc</span><span class="o">.</span><span class="n">ABC</span><span class="p">):</span>
</span><span id="CredentialProvider-87"><a href="#CredentialProvider-87"><span class="linenos"> 87</span></a><span class="w">    </span><span class="sd">"""Base class for credential providers.</span>
</span><span id="CredentialProvider-88"><a href="#CredentialProvider-88"><span class="linenos"> 88</span></a>
</span><span id="CredentialProvider-89"><a href="#CredentialProvider-89"><span class="linenos"> 89</span></a><span class="sd">    Subclasses implement ``retrieve_credentials_impl``. Results are cached and</span>
</span><span id="CredentialProvider-90"><a href="#CredentialProvider-90"><span class="linenos"> 90</span></a><span class="sd">    reused until five minutes before they expire.</span>
</span><span id="CredentialProvider-91"><a href="#CredentialProvider-91"><span class="linenos"> 91</span></a><span class="sd">    """</span>
</span><span id="CredentialProvider-92"><a href="#CredentialProvider-92"><span class="linenos"> 92</span></a>
</span><span id="CredentialProvider-93"><a href="#CredentialProvider-93"><span class="linenos"> 93</span></a>    <span class="k">def</span><span class="w"> </span><span class="fm">__init__</span><span class="p">(</span><span class="bp">self</span><span class="p">)</span> <span class="o">-&gt;</span> <span class="kc">None</span><span class="p">:</span>
</span><span id="CredentialProvider-94"><a href="#CredentialProvider-94"><span class="linenos"> 94</span></a>        <span class="bp">self</span><span class="o">.</span><span class="n">_cached</span><span class="p">:</span> <span class="n">CredentialProviderFunctionReturn</span> <span class="o">|</span> <span class="kc">None</span> <span class="o">=</span> <span class="kc">None</span>
</span><span id="CredentialProvider-95"><a href="#CredentialProvider-95"><span class="linenos"> 95</span></a>        <span class="bp">self</span><span class="o">.</span><span class="n">_lock</span> <span class="o">=</span> <span class="n">threading</span><span class="o">.</span><span class="n">Lock</span><span class="p">()</span>
</span><span id="CredentialProvider-96"><a href="#CredentialProvider-96"><span class="linenos"> 96</span></a>
</span><span id="CredentialProvider-97"><a href="#CredentialProvider-97"><span class="linenos"> 97</span></a>    <span class="k">def</span><span class="w"> </span><span class="fm">__call__</span><span class="p">(</span><span class="bp">self</span><span class="p">)</span> <span class="o">-&gt;</span> <span class="n">CredentialProviderFunctionReturn</span><span class="p">:</span>
</span><span id="CredentialProvider-98"><a href="#CredentialProvider-98"><span class="linenos"> 98</span></a>        <span class="k">with</span> <span class="bp">self</span><span class="o">.</span><span class="n">_lock</span><span class="p">:</span>
</span><span id="CredentialProvider-99"><a href="#CredentialProvider-99"><span class="linenos"> 99</span></a>            <span class="n">cached</span> <span class="o">=</span> <span class="bp">self</span><span class="o">.</span><span class="n">_cached</span>
</span><span id="CredentialProvider-100"><a href="#CredentialProvider-100"><span class="linenos">100</span></a>            <span class="k">if</span> <span class="n">cached</span> <span class="ow">is</span> <span class="kc">None</span> <span class="ow">or</span> <span class="p">(</span>
</span><span id="CredentialProvider-101"><a href="#CredentialProvider-101"><span class="linenos">101</span></a>                <span class="n">cached</span><span class="p">[</span><span class="mi">1</span><span class="p">]</span> <span class="ow">is</span> <span class="ow">not</span> <span class="kc">None</span>
</span><span id="CredentialProvider-102"><a href="#CredentialProvider-102"><span class="linenos">102</span></a>                <span class="ow">and</span> <span class="n">cached</span><span class="p">[</span><span class="mi">1</span><span class="p">]</span> <span class="o">-</span> <span class="n">_REFRESH_MARGIN_SECONDS</span> <span class="o">&lt;=</span> <span class="nb">int</span><span class="p">(</span><span class="n">time</span><span class="o">.</span><span class="n">time</span><span class="p">())</span>
</span><span id="CredentialProvider-103"><a href="#CredentialProvider-103"><span class="linenos">103</span></a>            <span class="p">):</span>
</span><span id="CredentialProvider-104"><a href="#CredentialProvider-104"><span class="linenos">104</span></a>                <span class="n">cached</span> <span class="o">=</span> <span class="bp">self</span><span class="o">.</span><span class="n">retrieve_credentials_impl</span><span class="p">()</span>
</span><span id="CredentialProvider-105"><a href="#CredentialProvider-105"><span class="linenos">105</span></a>                <span class="bp">self</span><span class="o">.</span><span class="n">_cached</span> <span class="o">=</span> <span class="n">cached</span>
</span><span id="CredentialProvider-106"><a href="#CredentialProvider-106"><span class="linenos">106</span></a>        <span class="n">credentials</span><span class="p">,</span> <span class="n">expiry</span> <span class="o">=</span> <span class="n">cached</span>
</span><span id="CredentialProvider-107"><a href="#CredentialProvider-107"><span class="linenos">107</span></a>        <span class="k">return</span> <span class="p">{</span><span class="o">**</span><span class="n">credentials</span><span class="p">},</span> <span class="n">expiry</span>
</span><span id="CredentialProvider-108"><a href="#CredentialProvider-108"><span class="linenos">108</span></a>
</span><span id="CredentialProvider-109"><a href="#CredentialProvider-109"><span class="linenos">109</span></a>    <span class="nd">@abc</span><span class="o">.</span><span class="n">abstractmethod</span>
</span><span id="CredentialProvider-110"><a href="#CredentialProvider-110"><span class="linenos">110</span></a>    <span class="k">def</span><span class="w"> </span><span class="nf">retrieve_credentials_impl</span><span class="p">(</span><span class="bp">self</span><span class="p">)</span> <span class="o">-&gt;</span> <span class="n">CredentialProviderFunctionReturn</span><span class="p">:</span>
</span><span id="CredentialProvider-111"><a href="#CredentialProvider-111"><span class="linenos">111</span></a><span class="w">        </span><span class="sd">"""Fetch new credentials, bypassing the cache."""</span>
</span><span id="CredentialProvider-112"><a href="#CredentialProvider-112"><span class="linenos">112</span></a>
</span><span id="CredentialProvider-113"><a href="#CredentialProvider-113"><span class="linenos">113</span></a>    <span class="k">def</span><span class="w"> </span><span class="nf">clear_cached_credentials</span><span class="p">(</span><span class="bp">self</span><span class="p">)</span> <span class="o">-&gt;</span> <span class="kc">None</span><span class="p">:</span>
</span><span id="CredentialProvider-114"><a href="#CredentialProvider-114"><span class="linenos">114</span></a><span class="w">        </span><span class="sd">"""Forget cached credentials, so the next call fetches new ones."""</span>
</span><span id="CredentialProvider-115"><a href="#CredentialProvider-115"><span class="linenos">115</span></a>        <span class="k">with</span> <span class="bp">self</span><span class="o">.</span><span class="n">_lock</span><span class="p">:</span>
</span><span id="CredentialProvider-116"><a href="#CredentialProvider-116"><span class="linenos">116</span></a>            <span class="bp">self</span><span class="o">.</span><span class="n">_cached</span> <span class="o">=</span> <span class="kc">None</span>
</span><span id="CredentialProvider-117"><a href="#CredentialProvider-117"><span class="linenos">117</span></a>
</span><span id="CredentialProvider-118"><a href="#CredentialProvider-118"><span class="linenos">118</span></a>    <span class="k">def</span><span class="w"> </span><span class="nf">__getstate__</span><span class="p">(</span><span class="bp">self</span><span class="p">)</span> <span class="o">-&gt;</span> <span class="nb">dict</span><span class="p">[</span><span class="nb">str</span><span class="p">,</span> <span class="n">Any</span><span class="p">]:</span>
</span><span id="CredentialProvider-119"><a href="#CredentialProvider-119"><span class="linenos">119</span></a>        <span class="c1"># Credentials are secrets: never pickle them. Locks can't be pickled.</span>
</span><span id="CredentialProvider-120"><a href="#CredentialProvider-120"><span class="linenos">120</span></a>        <span class="n">state</span> <span class="o">=</span> <span class="bp">self</span><span class="o">.</span><span class="vm">__dict__</span><span class="o">.</span><span class="n">copy</span><span class="p">()</span>
</span><span id="CredentialProvider-121"><a href="#CredentialProvider-121"><span class="linenos">121</span></a>        <span class="n">state</span><span class="p">[</span><span class="s2">"_cached"</span><span class="p">]</span> <span class="o">=</span> <span class="kc">None</span>
</span><span id="CredentialProvider-122"><a href="#CredentialProvider-122"><span class="linenos">122</span></a>        <span class="k">del</span> <span class="n">state</span><span class="p">[</span><span class="s2">"_lock"</span><span class="p">]</span>
</span><span id="CredentialProvider-123"><a href="#CredentialProvider-123"><span class="linenos">123</span></a>        <span class="k">return</span> <span class="n">state</span>
</span><span id="CredentialProvider-124"><a href="#CredentialProvider-124"><span class="linenos">124</span></a>
</span><span id="CredentialProvider-125"><a href="#CredentialProvider-125"><span class="linenos">125</span></a>    <span class="k">def</span><span class="w"> </span><span class="nf">__setstate__</span><span class="p">(</span><span class="bp">self</span><span class="p">,</span> <span class="n">state</span><span class="p">:</span> <span class="nb">dict</span><span class="p">[</span><span class="nb">str</span><span class="p">,</span> <span class="n">Any</span><span class="p">])</span> <span class="o">-&gt;</span> <span class="kc">None</span><span class="p">:</span>
</span><span id="CredentialProvider-126"><a href="#CredentialProvider-126"><span class="linenos">126</span></a>        <span class="bp">self</span><span class="o">.</span><span class="vm">__dict__</span><span class="o">.</span><span class="n">update</span><span class="p">(</span><span class="n">state</span><span class="p">)</span>
</span><span id="CredentialProvider-127"><a href="#CredentialProvider-127"><span class="linenos">127</span></a>        <span class="bp">self</span><span class="o">.</span><span class="n">_lock</span> <span class="o">=</span> <span class="n">threading</span><span class="o">.</span><span class="n">Lock</span><span class="p">()</span>
</span><span id="CredentialProvider-128"><a href="#CredentialProvider-128"><span class="linenos">128</span></a>
</span><span id="CredentialProvider-129"><a href="#CredentialProvider-129"><span class="linenos">129</span></a>    <span class="k">def</span><span class="w"> </span><span class="fm">__repr__</span><span class="p">(</span><span class="bp">self</span><span class="p">)</span> <span class="o">-&gt;</span> <span class="nb">str</span><span class="p">:</span>
</span><span id="CredentialProvider-130"><a href="#CredentialProvider-130"><span class="linenos">130</span></a>        <span class="k">return</span> <span class="sa">f</span><span class="s2">"</span><span class="si">{</span><span class="nb">type</span><span class="p">(</span><span class="bp">self</span><span class="p">)</span><span class="o">.</span><span class="vm">__name__</span><span class="si">}</span><span class="s2">()"</span>
</span></pre></div>


            <div class="docstring"><p>Base class for credential providers.</p>

<p>Subclasses implement <code><a href="#CredentialProvider.retrieve_credentials_impl">retrieve_credentials_impl</a></code>. Results are cached and
reused until five minutes before they expire.</p>
</div>


                            <div id="CredentialProvider.clear_cached_credentials" class="classattr">
                                        <input id="CredentialProvider.clear_cached_credentials-view-source" class="view-source-toggle-state" type="checkbox" aria-hidden="true" tabindex="-1">
<div class="attr function">
            
        <span class="def">def</span>
        <span class="name">clear_cached_credentials</span><span class="signature pdoc-code condensed">(<span class="param"><span class="bp">self</span></span><span class="return-annotation">) -> <span class="kc">None</span>:</span></span>

                <label class="view-source-button" for="CredentialProvider.clear_cached_credentials-view-source"><span>View Source</span></label>

    </div>
    <a class="headerlink" href="#CredentialProvider.clear_cached_credentials"></a>
            <div class="pdoc-code codehilite"><pre><span></span><span id="CredentialProvider.clear_cached_credentials-113"><a href="#CredentialProvider.clear_cached_credentials-113"><span class="linenos">113</span></a>    <span class="k">def</span><span class="w"> </span><span class="nf">clear_cached_credentials</span><span class="p">(</span><span class="bp">self</span><span class="p">)</span> <span class="o">-&gt;</span> <span class="kc">None</span><span class="p">:</span>
</span><span id="CredentialProvider.clear_cached_credentials-114"><a href="#CredentialProvider.clear_cached_credentials-114"><span class="linenos">114</span></a><span class="w">        </span><span class="sd">"""Forget cached credentials, so the next call fetches new ones."""</span>
</span><span id="CredentialProvider.clear_cached_credentials-115"><a href="#CredentialProvider.clear_cached_credentials-115"><span class="linenos">115</span></a>        <span class="k">with</span> <span class="bp">self</span><span class="o">.</span><span class="n">_lock</span><span class="p">:</span>
</span><span id="CredentialProvider.clear_cached_credentials-116"><a href="#CredentialProvider.clear_cached_credentials-116"><span class="linenos">116</span></a>            <span class="bp">self</span><span class="o">.</span><span class="n">_cached</span> <span class="o">=</span> <span class="kc">None</span>
</span></pre></div>


            <div class="docstring"><p>Forget cached credentials, so the next call fetches new ones.</p>
</div>


                            </div>
                            <div id="CredentialProvider.retrieve_credentials_impl" class="classattr">
                                        <input id="CredentialProvider.retrieve_credentials_impl-view-source" class="view-source-toggle-state" type="checkbox" aria-hidden="true" tabindex="-1">
<div class="attr function">
                    <div class="decorator decorator-abc.abstractmethod">@abc.abstractmethod</div>

        <span class="def">def</span>
        <span class="name">retrieve_credentials_impl</span><span class="signature pdoc-code condensed">(<span class="param"><span class="bp">self</span></span><span class="return-annotation">) -> <span class="nb">tuple</span><span class="p">[</span><span class="nb">dict</span><span class="p">[</span><span class="nb">str</span><span class="p">,</span> <span class="nb">str</span><span class="p">],</span> <span class="n">typing</span><span class="o">.</span><span class="n">Optional</span><span class="p">[</span><span class="nb">int</span><span class="p">]]</span>:</span></span>

                <label class="view-source-button" for="CredentialProvider.retrieve_credentials_impl-view-source"><span>View Source</span></label>

    </div>
    <a class="headerlink" href="#CredentialProvider.retrieve_credentials_impl"></a>
            <div class="pdoc-code codehilite"><pre><span></span><span id="CredentialProvider.retrieve_credentials_impl-109"><a href="#CredentialProvider.retrieve_credentials_impl-109"><span class="linenos">109</span></a>    <span class="nd">@abc</span><span class="o">.</span><span class="n">abstractmethod</span>
</span><span id="CredentialProvider.retrieve_credentials_impl-110"><a href="#CredentialProvider.retrieve_credentials_impl-110"><span class="linenos">110</span></a>    <span class="k">def</span><span class="w"> </span><span class="nf">retrieve_credentials_impl</span><span class="p">(</span><span class="bp">self</span><span class="p">)</span> <span class="o">-&gt;</span> <span class="n">CredentialProviderFunctionReturn</span><span class="p">:</span>
</span><span id="CredentialProvider.retrieve_credentials_impl-111"><a href="#CredentialProvider.retrieve_credentials_impl-111"><span class="linenos">111</span></a><span class="w">        </span><span class="sd">"""Fetch new credentials, bypassing the cache."""</span>
</span></pre></div>


            <div class="docstring"><p>Fetch new credentials, bypassing the cache.</p>
</div>


                            </div>
                </section>
</main>

## CredentialProviderAWS

<main class="pdoc">
<section id="CredentialProviderAWS">
                            <input id="CredentialProviderAWS-view-source" class="view-source-toggle-state" type="checkbox" aria-hidden="true" tabindex="-1">
<div class="attr class">
            
    <span class="def">class</span>
    <span class="name">CredentialProviderAWS</span><wbr>(<span class="base"><a href="#CredentialProvider">CredentialProvider</a></span>):

                <label class="view-source-button" for="CredentialProviderAWS-view-source"><span>View Source</span></label>

    </div>
    <a class="headerlink" href="#CredentialProviderAWS"></a>
            <div class="pdoc-code codehilite"><pre><span></span><span id="CredentialProviderAWS-138"><a href="#CredentialProviderAWS-138"><span class="linenos">138</span></a><span class="k">class</span><span class="w"> </span><span class="nc">CredentialProviderAWS</span><span class="p">(</span><span class="n">CredentialProvider</span><span class="p">):</span>
</span><span id="CredentialProviderAWS-139"><a href="#CredentialProviderAWS-139"><span class="linenos">139</span></a><span class="w">    </span><span class="sd">"""AWS credentials from boto3: profiles, SSO, assume-role, instance roles.</span>
</span><span id="CredentialProviderAWS-140"><a href="#CredentialProviderAWS-140"><span class="linenos">140</span></a>
</span><span id="CredentialProviderAWS-141"><a href="#CredentialProviderAWS-141"><span class="linenos">141</span></a><span class="sd">    Uses boto3's credential chain, so everything the AWS CLI supports works,</span>
</span><span id="CredentialProviderAWS-142"><a href="#CredentialProviderAWS-142"><span class="linenos">142</span></a><span class="sd">    including ``AWS_PROFILE`` and SSO sessions. Requires ``boto3``.</span>
</span><span id="CredentialProviderAWS-143"><a href="#CredentialProviderAWS-143"><span class="linenos">143</span></a>
</span><span id="CredentialProviderAWS-144"><a href="#CredentialProviderAWS-144"><span class="linenos">144</span></a><span class="sd">    Args:</span>
</span><span id="CredentialProviderAWS-145"><a href="#CredentialProviderAWS-145"><span class="linenos">145</span></a><span class="sd">        profile_name: Profile to use from ``~/.aws/config``.</span>
</span><span id="CredentialProviderAWS-146"><a href="#CredentialProviderAWS-146"><span class="linenos">146</span></a><span class="sd">        region_name: Region to use; defaults to the profile's region.</span>
</span><span id="CredentialProviderAWS-147"><a href="#CredentialProviderAWS-147"><span class="linenos">147</span></a><span class="sd">        assume_role: If given, assume this role with the session's</span>
</span><span id="CredentialProviderAWS-148"><a href="#CredentialProviderAWS-148"><span class="linenos">148</span></a><span class="sd">            credentials. The keys are passed to ``STS.Client.assume_role()``,</span>
</span><span id="CredentialProviderAWS-149"><a href="#CredentialProviderAWS-149"><span class="linenos">149</span></a><span class="sd">            e.g. ``{"RoleArn": "...", "RoleSessionName": "tantivy"}``.</span>
</span><span id="CredentialProviderAWS-150"><a href="#CredentialProviderAWS-150"><span class="linenos">150</span></a><span class="sd">    """</span>
</span><span id="CredentialProviderAWS-151"><a href="#CredentialProviderAWS-151"><span class="linenos">151</span></a>
</span><span id="CredentialProviderAWS-152"><a href="#CredentialProviderAWS-152"><span class="linenos">152</span></a>    <span class="k">def</span><span class="w"> </span><span class="fm">__init__</span><span class="p">(</span>
</span><span id="CredentialProviderAWS-153"><a href="#CredentialProviderAWS-153"><span class="linenos">153</span></a>        <span class="bp">self</span><span class="p">,</span>
</span><span id="CredentialProviderAWS-154"><a href="#CredentialProviderAWS-154"><span class="linenos">154</span></a>        <span class="o">*</span><span class="p">,</span>
</span><span id="CredentialProviderAWS-155"><a href="#CredentialProviderAWS-155"><span class="linenos">155</span></a>        <span class="n">profile_name</span><span class="p">:</span> <span class="nb">str</span> <span class="o">|</span> <span class="kc">None</span> <span class="o">=</span> <span class="kc">None</span><span class="p">,</span>
</span><span id="CredentialProviderAWS-156"><a href="#CredentialProviderAWS-156"><span class="linenos">156</span></a>        <span class="n">region_name</span><span class="p">:</span> <span class="nb">str</span> <span class="o">|</span> <span class="kc">None</span> <span class="o">=</span> <span class="kc">None</span><span class="p">,</span>
</span><span id="CredentialProviderAWS-157"><a href="#CredentialProviderAWS-157"><span class="linenos">157</span></a>        <span class="n">assume_role</span><span class="p">:</span> <span class="n">AWSAssumeRoleKWArgs</span> <span class="o">|</span> <span class="kc">None</span> <span class="o">=</span> <span class="kc">None</span><span class="p">,</span>
</span><span id="CredentialProviderAWS-158"><a href="#CredentialProviderAWS-158"><span class="linenos">158</span></a>    <span class="p">)</span> <span class="o">-&gt;</span> <span class="kc">None</span><span class="p">:</span>
</span><span id="CredentialProviderAWS-159"><a href="#CredentialProviderAWS-159"><span class="linenos">159</span></a>        <span class="n">_require</span><span class="p">(</span><span class="s2">"boto3"</span><span class="p">,</span> <span class="s2">"boto3"</span><span class="p">,</span> <span class="s2">"CredentialProviderAWS"</span><span class="p">)</span>
</span><span id="CredentialProviderAWS-160"><a href="#CredentialProviderAWS-160"><span class="linenos">160</span></a>        <span class="bp">self</span><span class="o">.</span><span class="n">profile_name</span> <span class="o">=</span> <span class="n">profile_name</span>
</span><span id="CredentialProviderAWS-161"><a href="#CredentialProviderAWS-161"><span class="linenos">161</span></a>        <span class="bp">self</span><span class="o">.</span><span class="n">region_name</span> <span class="o">=</span> <span class="n">region_name</span>
</span><span id="CredentialProviderAWS-162"><a href="#CredentialProviderAWS-162"><span class="linenos">162</span></a>        <span class="bp">self</span><span class="o">.</span><span class="n">assume_role</span> <span class="o">=</span> <span class="n">assume_role</span>
</span><span id="CredentialProviderAWS-163"><a href="#CredentialProviderAWS-163"><span class="linenos">163</span></a>        <span class="nb">super</span><span class="p">()</span><span class="o">.</span><span class="fm">__init__</span><span class="p">()</span>
</span><span id="CredentialProviderAWS-164"><a href="#CredentialProviderAWS-164"><span class="linenos">164</span></a>
</span><span id="CredentialProviderAWS-165"><a href="#CredentialProviderAWS-165"><span class="linenos">165</span></a>    <span class="k">def</span><span class="w"> </span><span class="nf">retrieve_credentials_impl</span><span class="p">(</span><span class="bp">self</span><span class="p">)</span> <span class="o">-&gt;</span> <span class="n">CredentialProviderFunctionReturn</span><span class="p">:</span>
</span><span id="CredentialProviderAWS-166"><a href="#CredentialProviderAWS-166"><span class="linenos">166</span></a>        <span class="n">session</span> <span class="o">=</span> <span class="bp">self</span><span class="o">.</span><span class="n">_session</span><span class="p">()</span>
</span><span id="CredentialProviderAWS-167"><a href="#CredentialProviderAWS-167"><span class="linenos">167</span></a>        <span class="k">if</span> <span class="bp">self</span><span class="o">.</span><span class="n">assume_role</span> <span class="ow">is</span> <span class="ow">not</span> <span class="kc">None</span><span class="p">:</span>
</span><span id="CredentialProviderAWS-168"><a href="#CredentialProviderAWS-168"><span class="linenos">168</span></a>            <span class="k">return</span> <span class="bp">self</span><span class="o">.</span><span class="n">_assume_role</span><span class="p">(</span><span class="n">session</span><span class="p">)</span>
</span><span id="CredentialProviderAWS-169"><a href="#CredentialProviderAWS-169"><span class="linenos">169</span></a>
</span><span id="CredentialProviderAWS-170"><a href="#CredentialProviderAWS-170"><span class="linenos">170</span></a>        <span class="n">creds</span> <span class="o">=</span> <span class="n">session</span><span class="o">.</span><span class="n">get_credentials</span><span class="p">()</span>
</span><span id="CredentialProviderAWS-171"><a href="#CredentialProviderAWS-171"><span class="linenos">171</span></a>        <span class="k">if</span> <span class="n">creds</span> <span class="ow">is</span> <span class="kc">None</span><span class="p">:</span>
</span><span id="CredentialProviderAWS-172"><a href="#CredentialProviderAWS-172"><span class="linenos">172</span></a>            <span class="k">raise</span> <span class="ne">RuntimeError</span><span class="p">(</span>
</span><span id="CredentialProviderAWS-173"><a href="#CredentialProviderAWS-173"><span class="linenos">173</span></a>                <span class="s2">"boto3 found no AWS credentials; configure a profile, "</span>
</span><span id="CredentialProviderAWS-174"><a href="#CredentialProviderAWS-174"><span class="linenos">174</span></a>                <span class="s2">"environment variables or an instance role"</span>
</span><span id="CredentialProviderAWS-175"><a href="#CredentialProviderAWS-175"><span class="linenos">175</span></a>            <span class="p">)</span>
</span><span id="CredentialProviderAWS-176"><a href="#CredentialProviderAWS-176"><span class="linenos">176</span></a>        <span class="c1"># Read the keys before the expiry: for refreshable credentials,</span>
</span><span id="CredentialProviderAWS-177"><a href="#CredentialProviderAWS-177"><span class="linenos">177</span></a>        <span class="c1"># reading them is what triggers the refresh that sets the expiry.</span>
</span><span id="CredentialProviderAWS-178"><a href="#CredentialProviderAWS-178"><span class="linenos">178</span></a>        <span class="n">frozen</span> <span class="o">=</span> <span class="n">creds</span><span class="o">.</span><span class="n">get_frozen_credentials</span><span class="p">()</span>
</span><span id="CredentialProviderAWS-179"><a href="#CredentialProviderAWS-179"><span class="linenos">179</span></a>        <span class="n">credentials</span> <span class="o">=</span> <span class="p">{</span>
</span><span id="CredentialProviderAWS-180"><a href="#CredentialProviderAWS-180"><span class="linenos">180</span></a>            <span class="s2">"aws_access_key_id"</span><span class="p">:</span> <span class="n">frozen</span><span class="o">.</span><span class="n">access_key</span><span class="p">,</span>
</span><span id="CredentialProviderAWS-181"><a href="#CredentialProviderAWS-181"><span class="linenos">181</span></a>            <span class="s2">"aws_secret_access_key"</span><span class="p">:</span> <span class="n">frozen</span><span class="o">.</span><span class="n">secret_key</span><span class="p">,</span>
</span><span id="CredentialProviderAWS-182"><a href="#CredentialProviderAWS-182"><span class="linenos">182</span></a>        <span class="p">}</span>
</span><span id="CredentialProviderAWS-183"><a href="#CredentialProviderAWS-183"><span class="linenos">183</span></a>        <span class="k">if</span> <span class="n">frozen</span><span class="o">.</span><span class="n">token</span> <span class="ow">is</span> <span class="ow">not</span> <span class="kc">None</span><span class="p">:</span>
</span><span id="CredentialProviderAWS-184"><a href="#CredentialProviderAWS-184"><span class="linenos">184</span></a>            <span class="n">credentials</span><span class="p">[</span><span class="s2">"aws_session_token"</span><span class="p">]</span> <span class="o">=</span> <span class="n">frozen</span><span class="o">.</span><span class="n">token</span>
</span><span id="CredentialProviderAWS-185"><a href="#CredentialProviderAWS-185"><span class="linenos">185</span></a>        <span class="n">expiry</span> <span class="o">=</span> <span class="nb">getattr</span><span class="p">(</span><span class="n">creds</span><span class="p">,</span> <span class="s2">"_expiry_time"</span><span class="p">,</span> <span class="kc">None</span><span class="p">)</span>
</span><span id="CredentialProviderAWS-186"><a href="#CredentialProviderAWS-186"><span class="linenos">186</span></a>        <span class="k">return</span> <span class="n">credentials</span><span class="p">,</span> <span class="p">(</span>
</span><span id="CredentialProviderAWS-187"><a href="#CredentialProviderAWS-187"><span class="linenos">187</span></a>            <span class="nb">int</span><span class="p">(</span><span class="n">expiry</span><span class="o">.</span><span class="n">timestamp</span><span class="p">())</span> <span class="k">if</span> <span class="nb">isinstance</span><span class="p">(</span><span class="n">expiry</span><span class="p">,</span> <span class="n">datetime</span><span class="p">)</span> <span class="k">else</span> <span class="kc">None</span>
</span><span id="CredentialProviderAWS-188"><a href="#CredentialProviderAWS-188"><span class="linenos">188</span></a>        <span class="p">)</span>
</span><span id="CredentialProviderAWS-189"><a href="#CredentialProviderAWS-189"><span class="linenos">189</span></a>
</span><span id="CredentialProviderAWS-190"><a href="#CredentialProviderAWS-190"><span class="linenos">190</span></a>    <span class="k">def</span><span class="w"> </span><span class="nf">_assume_role</span><span class="p">(</span><span class="bp">self</span><span class="p">,</span> <span class="n">session</span><span class="p">:</span> <span class="n">Any</span><span class="p">)</span> <span class="o">-&gt;</span> <span class="n">CredentialProviderFunctionReturn</span><span class="p">:</span>
</span><span id="CredentialProviderAWS-191"><a href="#CredentialProviderAWS-191"><span class="linenos">191</span></a>        <span class="k">assert</span> <span class="bp">self</span><span class="o">.</span><span class="n">assume_role</span> <span class="ow">is</span> <span class="ow">not</span> <span class="kc">None</span>
</span><span id="CredentialProviderAWS-192"><a href="#CredentialProviderAWS-192"><span class="linenos">192</span></a>        <span class="n">response</span> <span class="o">=</span> <span class="n">session</span><span class="o">.</span><span class="n">client</span><span class="p">(</span><span class="s2">"sts"</span><span class="p">)</span><span class="o">.</span><span class="n">assume_role</span><span class="p">(</span><span class="o">**</span><span class="bp">self</span><span class="o">.</span><span class="n">assume_role</span><span class="p">)</span>
</span><span id="CredentialProviderAWS-193"><a href="#CredentialProviderAWS-193"><span class="linenos">193</span></a>        <span class="n">creds</span> <span class="o">=</span> <span class="n">response</span><span class="p">[</span><span class="s2">"Credentials"</span><span class="p">]</span>
</span><span id="CredentialProviderAWS-194"><a href="#CredentialProviderAWS-194"><span class="linenos">194</span></a>        <span class="n">expiry</span> <span class="o">=</span> <span class="n">creds</span><span class="p">[</span><span class="s2">"Expiration"</span><span class="p">]</span>
</span><span id="CredentialProviderAWS-195"><a href="#CredentialProviderAWS-195"><span class="linenos">195</span></a>        <span class="k">if</span> <span class="n">expiry</span><span class="o">.</span><span class="n">tzinfo</span> <span class="ow">is</span> <span class="kc">None</span><span class="p">:</span>
</span><span id="CredentialProviderAWS-196"><a href="#CredentialProviderAWS-196"><span class="linenos">196</span></a>            <span class="k">raise</span> <span class="ne">ValueError</span><span class="p">(</span><span class="s2">"STS returned an expiration time with no time zone"</span><span class="p">)</span>
</span><span id="CredentialProviderAWS-197"><a href="#CredentialProviderAWS-197"><span class="linenos">197</span></a>        <span class="k">return</span> <span class="p">{</span>
</span><span id="CredentialProviderAWS-198"><a href="#CredentialProviderAWS-198"><span class="linenos">198</span></a>            <span class="s2">"aws_access_key_id"</span><span class="p">:</span> <span class="n">creds</span><span class="p">[</span><span class="s2">"AccessKeyId"</span><span class="p">],</span>
</span><span id="CredentialProviderAWS-199"><a href="#CredentialProviderAWS-199"><span class="linenos">199</span></a>            <span class="s2">"aws_secret_access_key"</span><span class="p">:</span> <span class="n">creds</span><span class="p">[</span><span class="s2">"SecretAccessKey"</span><span class="p">],</span>
</span><span id="CredentialProviderAWS-200"><a href="#CredentialProviderAWS-200"><span class="linenos">200</span></a>            <span class="s2">"aws_session_token"</span><span class="p">:</span> <span class="n">creds</span><span class="p">[</span><span class="s2">"SessionToken"</span><span class="p">],</span>
</span><span id="CredentialProviderAWS-201"><a href="#CredentialProviderAWS-201"><span class="linenos">201</span></a>        <span class="p">},</span> <span class="nb">int</span><span class="p">(</span><span class="n">expiry</span><span class="o">.</span><span class="n">timestamp</span><span class="p">())</span>
</span><span id="CredentialProviderAWS-202"><a href="#CredentialProviderAWS-202"><span class="linenos">202</span></a>
</span><span id="CredentialProviderAWS-203"><a href="#CredentialProviderAWS-203"><span class="linenos">203</span></a>    <span class="k">def</span><span class="w"> </span><span class="nf">_storage_options</span><span class="p">(</span><span class="bp">self</span><span class="p">)</span> <span class="o">-&gt;</span> <span class="nb">dict</span><span class="p">[</span><span class="nb">str</span><span class="p">,</span> <span class="nb">str</span><span class="p">]:</span>
</span><span id="CredentialProviderAWS-204"><a href="#CredentialProviderAWS-204"><span class="linenos">204</span></a><span class="w">        </span><span class="sd">"""Settings tantivy uses unless storage_options gives them.</span>
</span><span id="CredentialProviderAWS-205"><a href="#CredentialProviderAWS-205"><span class="linenos">205</span></a>
</span><span id="CredentialProviderAWS-206"><a href="#CredentialProviderAWS-206"><span class="linenos">206</span></a><span class="sd">        Called from Rust when the store is built: passes on the profile's</span>
</span><span id="CredentialProviderAWS-207"><a href="#CredentialProviderAWS-207"><span class="linenos">207</span></a><span class="sd">        region, which object_store cannot read for a named profile.</span>
</span><span id="CredentialProviderAWS-208"><a href="#CredentialProviderAWS-208"><span class="linenos">208</span></a><span class="sd">        """</span>
</span><span id="CredentialProviderAWS-209"><a href="#CredentialProviderAWS-209"><span class="linenos">209</span></a>        <span class="n">region</span> <span class="o">=</span> <span class="bp">self</span><span class="o">.</span><span class="n">region_name</span> <span class="ow">or</span> <span class="bp">self</span><span class="o">.</span><span class="n">_session</span><span class="p">()</span><span class="o">.</span><span class="n">region_name</span>
</span><span id="CredentialProviderAWS-210"><a href="#CredentialProviderAWS-210"><span class="linenos">210</span></a>        <span class="k">return</span> <span class="p">{</span><span class="s2">"region"</span><span class="p">:</span> <span class="n">region</span><span class="p">}</span> <span class="k">if</span> <span class="n">region</span> <span class="k">else</span> <span class="p">{}</span>
</span><span id="CredentialProviderAWS-211"><a href="#CredentialProviderAWS-211"><span class="linenos">211</span></a>
</span><span id="CredentialProviderAWS-212"><a href="#CredentialProviderAWS-212"><span class="linenos">212</span></a>    <span class="k">def</span><span class="w"> </span><span class="nf">_session</span><span class="p">(</span><span class="bp">self</span><span class="p">)</span> <span class="o">-&gt;</span> <span class="n">Any</span><span class="p">:</span>
</span><span id="CredentialProviderAWS-213"><a href="#CredentialProviderAWS-213"><span class="linenos">213</span></a>        <span class="kn">import</span><span class="w"> </span><span class="nn">boto3</span>
</span><span id="CredentialProviderAWS-214"><a href="#CredentialProviderAWS-214"><span class="linenos">214</span></a>
</span><span id="CredentialProviderAWS-215"><a href="#CredentialProviderAWS-215"><span class="linenos">215</span></a>        <span class="c1"># boto3 reads AWS_PROFILE itself when profile_name is None.</span>
</span><span id="CredentialProviderAWS-216"><a href="#CredentialProviderAWS-216"><span class="linenos">216</span></a>        <span class="k">return</span> <span class="n">boto3</span><span class="o">.</span><span class="n">Session</span><span class="p">(</span>
</span><span id="CredentialProviderAWS-217"><a href="#CredentialProviderAWS-217"><span class="linenos">217</span></a>            <span class="n">profile_name</span><span class="o">=</span><span class="bp">self</span><span class="o">.</span><span class="n">profile_name</span><span class="p">,</span> <span class="n">region_name</span><span class="o">=</span><span class="bp">self</span><span class="o">.</span><span class="n">region_name</span>
</span><span id="CredentialProviderAWS-218"><a href="#CredentialProviderAWS-218"><span class="linenos">218</span></a>        <span class="p">)</span>
</span><span id="CredentialProviderAWS-219"><a href="#CredentialProviderAWS-219"><span class="linenos">219</span></a>
</span><span id="CredentialProviderAWS-220"><a href="#CredentialProviderAWS-220"><span class="linenos">220</span></a>    <span class="k">def</span><span class="w"> </span><span class="fm">__repr__</span><span class="p">(</span><span class="bp">self</span><span class="p">)</span> <span class="o">-&gt;</span> <span class="nb">str</span><span class="p">:</span>
</span><span id="CredentialProviderAWS-221"><a href="#CredentialProviderAWS-221"><span class="linenos">221</span></a>        <span class="k">return</span> <span class="sa">f</span><span class="s2">"CredentialProviderAWS(profile_name=</span><span class="si">{</span><span class="bp">self</span><span class="o">.</span><span class="n">profile_name</span><span class="si">!r}</span><span class="s2">)"</span>
</span></pre></div>


            <div class="docstring"><p>AWS credentials from boto3: profiles, SSO, assume-role, instance roles.</p>

<p>Uses boto3's credential chain, so everything the AWS CLI supports works,
including <code>AWS_PROFILE</code> and SSO sessions. Requires <code>boto3</code>.</p>

<h6 id="arguments">Arguments:</h6>

<ul>
<li><strong>profile_name:</strong>  Profile to use from <code>~/.aws/config</code>.</li>
<li><strong>region_name:</strong>  Region to use; defaults to the profile's region.</li>
<li><strong>assume_role:</strong>  If given, assume this role with the session's
credentials. The keys are passed to <code>STS.Client.assume_role()</code>,
e.g. <code>{"RoleArn": "...", "RoleSessionName": "tantivy"}</code>.</li>
</ul>
</div>


                            <div id="CredentialProviderAWS.__init__" class="classattr">
                                        <input id="CredentialProviderAWS.__init__-view-source" class="view-source-toggle-state" type="checkbox" aria-hidden="true" tabindex="-1">
<div class="attr function">
            
        <span class="name">CredentialProviderAWS</span><span class="signature pdoc-code multiline">(<span class="param">    <span class="o">*</span>,</span><span class="param">    <span class="n">profile_name</span><span class="p">:</span> <span class="nb">str</span> <span class="o">|</span> <span class="kc">None</span> <span class="o">=</span> <span class="kc">None</span>,</span><span class="param">    <span class="n">region_name</span><span class="p">:</span> <span class="nb">str</span> <span class="o">|</span> <span class="kc">None</span> <span class="o">=</span> <span class="kc">None</span>,</span><span class="param">    <span class="n">assume_role</span><span class="p">:</span> <span class="n">tantivy</span><span class="o">.</span><span class="n">credential_provider</span><span class="o">.</span><span class="n">AWSAssumeRoleKWArgs</span> <span class="o">|</span> <span class="kc">None</span> <span class="o">=</span> <span class="kc">None</span></span>)</span>

                <label class="view-source-button" for="CredentialProviderAWS.__init__-view-source"><span>View Source</span></label>

    </div>
    <a class="headerlink" href="#CredentialProviderAWS.__init__"></a>
            <div class="pdoc-code codehilite"><pre><span></span><span id="CredentialProviderAWS.__init__-152"><a href="#CredentialProviderAWS.__init__-152"><span class="linenos">152</span></a>    <span class="k">def</span><span class="w"> </span><span class="fm">__init__</span><span class="p">(</span>
</span><span id="CredentialProviderAWS.__init__-153"><a href="#CredentialProviderAWS.__init__-153"><span class="linenos">153</span></a>        <span class="bp">self</span><span class="p">,</span>
</span><span id="CredentialProviderAWS.__init__-154"><a href="#CredentialProviderAWS.__init__-154"><span class="linenos">154</span></a>        <span class="o">*</span><span class="p">,</span>
</span><span id="CredentialProviderAWS.__init__-155"><a href="#CredentialProviderAWS.__init__-155"><span class="linenos">155</span></a>        <span class="n">profile_name</span><span class="p">:</span> <span class="nb">str</span> <span class="o">|</span> <span class="kc">None</span> <span class="o">=</span> <span class="kc">None</span><span class="p">,</span>
</span><span id="CredentialProviderAWS.__init__-156"><a href="#CredentialProviderAWS.__init__-156"><span class="linenos">156</span></a>        <span class="n">region_name</span><span class="p">:</span> <span class="nb">str</span> <span class="o">|</span> <span class="kc">None</span> <span class="o">=</span> <span class="kc">None</span><span class="p">,</span>
</span><span id="CredentialProviderAWS.__init__-157"><a href="#CredentialProviderAWS.__init__-157"><span class="linenos">157</span></a>        <span class="n">assume_role</span><span class="p">:</span> <span class="n">AWSAssumeRoleKWArgs</span> <span class="o">|</span> <span class="kc">None</span> <span class="o">=</span> <span class="kc">None</span><span class="p">,</span>
</span><span id="CredentialProviderAWS.__init__-158"><a href="#CredentialProviderAWS.__init__-158"><span class="linenos">158</span></a>    <span class="p">)</span> <span class="o">-&gt;</span> <span class="kc">None</span><span class="p">:</span>
</span><span id="CredentialProviderAWS.__init__-159"><a href="#CredentialProviderAWS.__init__-159"><span class="linenos">159</span></a>        <span class="n">_require</span><span class="p">(</span><span class="s2">"boto3"</span><span class="p">,</span> <span class="s2">"boto3"</span><span class="p">,</span> <span class="s2">"CredentialProviderAWS"</span><span class="p">)</span>
</span><span id="CredentialProviderAWS.__init__-160"><a href="#CredentialProviderAWS.__init__-160"><span class="linenos">160</span></a>        <span class="bp">self</span><span class="o">.</span><span class="n">profile_name</span> <span class="o">=</span> <span class="n">profile_name</span>
</span><span id="CredentialProviderAWS.__init__-161"><a href="#CredentialProviderAWS.__init__-161"><span class="linenos">161</span></a>        <span class="bp">self</span><span class="o">.</span><span class="n">region_name</span> <span class="o">=</span> <span class="n">region_name</span>
</span><span id="CredentialProviderAWS.__init__-162"><a href="#CredentialProviderAWS.__init__-162"><span class="linenos">162</span></a>        <span class="bp">self</span><span class="o">.</span><span class="n">assume_role</span> <span class="o">=</span> <span class="n">assume_role</span>
</span><span id="CredentialProviderAWS.__init__-163"><a href="#CredentialProviderAWS.__init__-163"><span class="linenos">163</span></a>        <span class="nb">super</span><span class="p">()</span><span class="o">.</span><span class="fm">__init__</span><span class="p">()</span>
</span></pre></div>


    

                            </div>
                            <div id="CredentialProviderAWS.assume_role" class="classattr">
                                <div class="attr variable">
            <span class="name">assume_role</span>

        
    </div>
    <a class="headerlink" href="#CredentialProviderAWS.assume_role"></a>
    
    

                            </div>
                            <div id="CredentialProviderAWS.profile_name" class="classattr">
                                <div class="attr variable">
            <span class="name">profile_name</span>

        
    </div>
    <a class="headerlink" href="#CredentialProviderAWS.profile_name"></a>
    
    

                            </div>
                            <div id="CredentialProviderAWS.region_name" class="classattr">
                                <div class="attr variable">
            <span class="name">region_name</span>

        
    </div>
    <a class="headerlink" href="#CredentialProviderAWS.region_name"></a>
    
    

                            </div>
                            <div id="CredentialProviderAWS.retrieve_credentials_impl" class="classattr">
                                        <input id="CredentialProviderAWS.retrieve_credentials_impl-view-source" class="view-source-toggle-state" type="checkbox" aria-hidden="true" tabindex="-1">
<div class="attr function">
            
        <span class="def">def</span>
        <span class="name">retrieve_credentials_impl</span><span class="signature pdoc-code condensed">(<span class="param"><span class="bp">self</span></span><span class="return-annotation">) -> <span class="nb">tuple</span><span class="p">[</span><span class="nb">dict</span><span class="p">[</span><span class="nb">str</span><span class="p">,</span> <span class="nb">str</span><span class="p">],</span> <span class="n">typing</span><span class="o">.</span><span class="n">Optional</span><span class="p">[</span><span class="nb">int</span><span class="p">]]</span>:</span></span>

                <label class="view-source-button" for="CredentialProviderAWS.retrieve_credentials_impl-view-source"><span>View Source</span></label>

    </div>
    <a class="headerlink" href="#CredentialProviderAWS.retrieve_credentials_impl"></a>
            <div class="pdoc-code codehilite"><pre><span></span><span id="CredentialProviderAWS.retrieve_credentials_impl-165"><a href="#CredentialProviderAWS.retrieve_credentials_impl-165"><span class="linenos">165</span></a>    <span class="k">def</span><span class="w"> </span><span class="nf">retrieve_credentials_impl</span><span class="p">(</span><span class="bp">self</span><span class="p">)</span> <span class="o">-&gt;</span> <span class="n">CredentialProviderFunctionReturn</span><span class="p">:</span>
</span><span id="CredentialProviderAWS.retrieve_credentials_impl-166"><a href="#CredentialProviderAWS.retrieve_credentials_impl-166"><span class="linenos">166</span></a>        <span class="n">session</span> <span class="o">=</span> <span class="bp">self</span><span class="o">.</span><span class="n">_session</span><span class="p">()</span>
</span><span id="CredentialProviderAWS.retrieve_credentials_impl-167"><a href="#CredentialProviderAWS.retrieve_credentials_impl-167"><span class="linenos">167</span></a>        <span class="k">if</span> <span class="bp">self</span><span class="o">.</span><span class="n">assume_role</span> <span class="ow">is</span> <span class="ow">not</span> <span class="kc">None</span><span class="p">:</span>
</span><span id="CredentialProviderAWS.retrieve_credentials_impl-168"><a href="#CredentialProviderAWS.retrieve_credentials_impl-168"><span class="linenos">168</span></a>            <span class="k">return</span> <span class="bp">self</span><span class="o">.</span><span class="n">_assume_role</span><span class="p">(</span><span class="n">session</span><span class="p">)</span>
</span><span id="CredentialProviderAWS.retrieve_credentials_impl-169"><a href="#CredentialProviderAWS.retrieve_credentials_impl-169"><span class="linenos">169</span></a>
</span><span id="CredentialProviderAWS.retrieve_credentials_impl-170"><a href="#CredentialProviderAWS.retrieve_credentials_impl-170"><span class="linenos">170</span></a>        <span class="n">creds</span> <span class="o">=</span> <span class="n">session</span><span class="o">.</span><span class="n">get_credentials</span><span class="p">()</span>
</span><span id="CredentialProviderAWS.retrieve_credentials_impl-171"><a href="#CredentialProviderAWS.retrieve_credentials_impl-171"><span class="linenos">171</span></a>        <span class="k">if</span> <span class="n">creds</span> <span class="ow">is</span> <span class="kc">None</span><span class="p">:</span>
</span><span id="CredentialProviderAWS.retrieve_credentials_impl-172"><a href="#CredentialProviderAWS.retrieve_credentials_impl-172"><span class="linenos">172</span></a>            <span class="k">raise</span> <span class="ne">RuntimeError</span><span class="p">(</span>
</span><span id="CredentialProviderAWS.retrieve_credentials_impl-173"><a href="#CredentialProviderAWS.retrieve_credentials_impl-173"><span class="linenos">173</span></a>                <span class="s2">"boto3 found no AWS credentials; configure a profile, "</span>
</span><span id="CredentialProviderAWS.retrieve_credentials_impl-174"><a href="#CredentialProviderAWS.retrieve_credentials_impl-174"><span class="linenos">174</span></a>                <span class="s2">"environment variables or an instance role"</span>
</span><span id="CredentialProviderAWS.retrieve_credentials_impl-175"><a href="#CredentialProviderAWS.retrieve_credentials_impl-175"><span class="linenos">175</span></a>            <span class="p">)</span>
</span><span id="CredentialProviderAWS.retrieve_credentials_impl-176"><a href="#CredentialProviderAWS.retrieve_credentials_impl-176"><span class="linenos">176</span></a>        <span class="c1"># Read the keys before the expiry: for refreshable credentials,</span>
</span><span id="CredentialProviderAWS.retrieve_credentials_impl-177"><a href="#CredentialProviderAWS.retrieve_credentials_impl-177"><span class="linenos">177</span></a>        <span class="c1"># reading them is what triggers the refresh that sets the expiry.</span>
</span><span id="CredentialProviderAWS.retrieve_credentials_impl-178"><a href="#CredentialProviderAWS.retrieve_credentials_impl-178"><span class="linenos">178</span></a>        <span class="n">frozen</span> <span class="o">=</span> <span class="n">creds</span><span class="o">.</span><span class="n">get_frozen_credentials</span><span class="p">()</span>
</span><span id="CredentialProviderAWS.retrieve_credentials_impl-179"><a href="#CredentialProviderAWS.retrieve_credentials_impl-179"><span class="linenos">179</span></a>        <span class="n">credentials</span> <span class="o">=</span> <span class="p">{</span>
</span><span id="CredentialProviderAWS.retrieve_credentials_impl-180"><a href="#CredentialProviderAWS.retrieve_credentials_impl-180"><span class="linenos">180</span></a>            <span class="s2">"aws_access_key_id"</span><span class="p">:</span> <span class="n">frozen</span><span class="o">.</span><span class="n">access_key</span><span class="p">,</span>
</span><span id="CredentialProviderAWS.retrieve_credentials_impl-181"><a href="#CredentialProviderAWS.retrieve_credentials_impl-181"><span class="linenos">181</span></a>            <span class="s2">"aws_secret_access_key"</span><span class="p">:</span> <span class="n">frozen</span><span class="o">.</span><span class="n">secret_key</span><span class="p">,</span>
</span><span id="CredentialProviderAWS.retrieve_credentials_impl-182"><a href="#CredentialProviderAWS.retrieve_credentials_impl-182"><span class="linenos">182</span></a>        <span class="p">}</span>
</span><span id="CredentialProviderAWS.retrieve_credentials_impl-183"><a href="#CredentialProviderAWS.retrieve_credentials_impl-183"><span class="linenos">183</span></a>        <span class="k">if</span> <span class="n">frozen</span><span class="o">.</span><span class="n">token</span> <span class="ow">is</span> <span class="ow">not</span> <span class="kc">None</span><span class="p">:</span>
</span><span id="CredentialProviderAWS.retrieve_credentials_impl-184"><a href="#CredentialProviderAWS.retrieve_credentials_impl-184"><span class="linenos">184</span></a>            <span class="n">credentials</span><span class="p">[</span><span class="s2">"aws_session_token"</span><span class="p">]</span> <span class="o">=</span> <span class="n">frozen</span><span class="o">.</span><span class="n">token</span>
</span><span id="CredentialProviderAWS.retrieve_credentials_impl-185"><a href="#CredentialProviderAWS.retrieve_credentials_impl-185"><span class="linenos">185</span></a>        <span class="n">expiry</span> <span class="o">=</span> <span class="nb">getattr</span><span class="p">(</span><span class="n">creds</span><span class="p">,</span> <span class="s2">"_expiry_time"</span><span class="p">,</span> <span class="kc">None</span><span class="p">)</span>
</span><span id="CredentialProviderAWS.retrieve_credentials_impl-186"><a href="#CredentialProviderAWS.retrieve_credentials_impl-186"><span class="linenos">186</span></a>        <span class="k">return</span> <span class="n">credentials</span><span class="p">,</span> <span class="p">(</span>
</span><span id="CredentialProviderAWS.retrieve_credentials_impl-187"><a href="#CredentialProviderAWS.retrieve_credentials_impl-187"><span class="linenos">187</span></a>            <span class="nb">int</span><span class="p">(</span><span class="n">expiry</span><span class="o">.</span><span class="n">timestamp</span><span class="p">())</span> <span class="k">if</span> <span class="nb">isinstance</span><span class="p">(</span><span class="n">expiry</span><span class="p">,</span> <span class="n">datetime</span><span class="p">)</span> <span class="k">else</span> <span class="kc">None</span>
</span><span id="CredentialProviderAWS.retrieve_credentials_impl-188"><a href="#CredentialProviderAWS.retrieve_credentials_impl-188"><span class="linenos">188</span></a>        <span class="p">)</span>
</span></pre></div>


            <div class="docstring"><p>Fetch new credentials, bypassing the cache.</p>
</div>


                            </div>
                            <div class="inherited">
                                <h5>Inherited Members</h5>
                                <dl>
                                    <div><dt><a href="#CredentialProvider">CredentialProvider</a></dt>
                                <dd id="CredentialProviderAWS.clear_cached_credentials" class="function"><a href="#CredentialProvider.clear_cached_credentials">clear_cached_credentials</a></dd>

            </div>
                                </dl>
                            </div>
                </section>
</main>

## CredentialProviderAzure

<main class="pdoc">
<section id="CredentialProviderAzure">
                            <input id="CredentialProviderAzure-view-source" class="view-source-toggle-state" type="checkbox" aria-hidden="true" tabindex="-1">
<div class="attr class">
            
    <span class="def">class</span>
    <span class="name">CredentialProviderAzure</span><wbr>(<span class="base"><a href="#CredentialProvider">CredentialProvider</a></span>):

                <label class="view-source-button" for="CredentialProviderAzure-view-source"><span>View Source</span></label>

    </div>
    <a class="headerlink" href="#CredentialProviderAzure"></a>
            <div class="pdoc-code codehilite"><pre><span></span><span id="CredentialProviderAzure-272"><a href="#CredentialProviderAzure-272"><span class="linenos">272</span></a><span class="k">class</span><span class="w"> </span><span class="nc">CredentialProviderAzure</span><span class="p">(</span><span class="n">CredentialProvider</span><span class="p">):</span>
</span><span id="CredentialProviderAzure-273"><a href="#CredentialProviderAzure-273"><span class="linenos">273</span></a><span class="w">    </span><span class="sd">"""Azure credentials from ``azure.identity``.</span>
</span><span id="CredentialProviderAzure-274"><a href="#CredentialProviderAzure-274"><span class="linenos">274</span></a>
</span><span id="CredentialProviderAzure-275"><a href="#CredentialProviderAzure-275"><span class="linenos">275</span></a><span class="sd">    By default uses ``DefaultAzureCredential()``, which covers environment</span>
</span><span id="CredentialProviderAzure-276"><a href="#CredentialProviderAzure-276"><span class="linenos">276</span></a><span class="sd">    variables, managed identity and ``az login``. Requires ``azure-identity``</span>
</span><span id="CredentialProviderAzure-277"><a href="#CredentialProviderAzure-277"><span class="linenos">277</span></a><span class="sd">    unless ``credential`` is given.</span>
</span><span id="CredentialProviderAzure-278"><a href="#CredentialProviderAzure-278"><span class="linenos">278</span></a>
</span><span id="CredentialProviderAzure-279"><a href="#CredentialProviderAzure-279"><span class="linenos">279</span></a><span class="sd">    Args:</span>
</span><span id="CredentialProviderAzure-280"><a href="#CredentialProviderAzure-280"><span class="linenos">280</span></a><span class="sd">        scopes: Scopes to request a token for.</span>
</span><span id="CredentialProviderAzure-281"><a href="#CredentialProviderAzure-281"><span class="linenos">281</span></a><span class="sd">        tenant_id: Azure tenant ID.</span>
</span><span id="CredentialProviderAzure-282"><a href="#CredentialProviderAzure-282"><span class="linenos">282</span></a><span class="sd">        credential: An Azure credential object to use instead, such as</span>
</span><span id="CredentialProviderAzure-283"><a href="#CredentialProviderAzure-283"><span class="linenos">283</span></a><span class="sd">            ``azure.identity.ClientSecretCredential(...)``. It must have a</span>
</span><span id="CredentialProviderAzure-284"><a href="#CredentialProviderAzure-284"><span class="linenos">284</span></a><span class="sd">            ``get_token()`` method.</span>
</span><span id="CredentialProviderAzure-285"><a href="#CredentialProviderAzure-285"><span class="linenos">285</span></a><span class="sd">    """</span>
</span><span id="CredentialProviderAzure-286"><a href="#CredentialProviderAzure-286"><span class="linenos">286</span></a>
</span><span id="CredentialProviderAzure-287"><a href="#CredentialProviderAzure-287"><span class="linenos">287</span></a>    <span class="k">def</span><span class="w"> </span><span class="fm">__init__</span><span class="p">(</span>
</span><span id="CredentialProviderAzure-288"><a href="#CredentialProviderAzure-288"><span class="linenos">288</span></a>        <span class="bp">self</span><span class="p">,</span>
</span><span id="CredentialProviderAzure-289"><a href="#CredentialProviderAzure-289"><span class="linenos">289</span></a>        <span class="o">*</span><span class="p">,</span>
</span><span id="CredentialProviderAzure-290"><a href="#CredentialProviderAzure-290"><span class="linenos">290</span></a>        <span class="n">scopes</span><span class="p">:</span> <span class="nb">list</span><span class="p">[</span><span class="nb">str</span><span class="p">]</span> <span class="o">|</span> <span class="kc">None</span> <span class="o">=</span> <span class="kc">None</span><span class="p">,</span>
</span><span id="CredentialProviderAzure-291"><a href="#CredentialProviderAzure-291"><span class="linenos">291</span></a>        <span class="n">tenant_id</span><span class="p">:</span> <span class="nb">str</span> <span class="o">|</span> <span class="kc">None</span> <span class="o">=</span> <span class="kc">None</span><span class="p">,</span>
</span><span id="CredentialProviderAzure-292"><a href="#CredentialProviderAzure-292"><span class="linenos">292</span></a>        <span class="n">credential</span><span class="p">:</span> <span class="n">Any</span> <span class="o">|</span> <span class="kc">None</span> <span class="o">=</span> <span class="kc">None</span><span class="p">,</span>
</span><span id="CredentialProviderAzure-293"><a href="#CredentialProviderAzure-293"><span class="linenos">293</span></a>    <span class="p">)</span> <span class="o">-&gt;</span> <span class="kc">None</span><span class="p">:</span>
</span><span id="CredentialProviderAzure-294"><a href="#CredentialProviderAzure-294"><span class="linenos">294</span></a>        <span class="k">if</span> <span class="n">credential</span> <span class="ow">is</span> <span class="kc">None</span><span class="p">:</span>
</span><span id="CredentialProviderAzure-295"><a href="#CredentialProviderAzure-295"><span class="linenos">295</span></a>            <span class="n">_require</span><span class="p">(</span><span class="s2">"azure.identity"</span><span class="p">,</span> <span class="s2">"azure-identity"</span><span class="p">,</span> <span class="s2">"CredentialProviderAzure"</span><span class="p">)</span>
</span><span id="CredentialProviderAzure-296"><a href="#CredentialProviderAzure-296"><span class="linenos">296</span></a>        <span class="k">elif</span> <span class="ow">not</span> <span class="nb">hasattr</span><span class="p">(</span><span class="n">credential</span><span class="p">,</span> <span class="s2">"get_token"</span><span class="p">):</span>
</span><span id="CredentialProviderAzure-297"><a href="#CredentialProviderAzure-297"><span class="linenos">297</span></a>            <span class="k">raise</span> <span class="ne">ValueError</span><span class="p">(</span>
</span><span id="CredentialProviderAzure-298"><a href="#CredentialProviderAzure-298"><span class="linenos">298</span></a>                <span class="sa">f</span><span class="s2">"credential </span><span class="si">{</span><span class="n">credential</span><span class="si">!r}</span><span class="s2"> does not have a get_token() method"</span>
</span><span id="CredentialProviderAzure-299"><a href="#CredentialProviderAzure-299"><span class="linenos">299</span></a>            <span class="p">)</span>
</span><span id="CredentialProviderAzure-300"><a href="#CredentialProviderAzure-300"><span class="linenos">300</span></a>        <span class="bp">self</span><span class="o">.</span><span class="n">scopes</span> <span class="o">=</span> <span class="p">(</span>
</span><span id="CredentialProviderAzure-301"><a href="#CredentialProviderAzure-301"><span class="linenos">301</span></a>            <span class="n">scopes</span> <span class="k">if</span> <span class="n">scopes</span> <span class="ow">is</span> <span class="ow">not</span> <span class="kc">None</span> <span class="k">else</span> <span class="p">[</span><span class="s2">"https://storage.azure.com/.default"</span><span class="p">]</span>
</span><span id="CredentialProviderAzure-302"><a href="#CredentialProviderAzure-302"><span class="linenos">302</span></a>        <span class="p">)</span>
</span><span id="CredentialProviderAzure-303"><a href="#CredentialProviderAzure-303"><span class="linenos">303</span></a>        <span class="bp">self</span><span class="o">.</span><span class="n">tenant_id</span> <span class="o">=</span> <span class="n">tenant_id</span>
</span><span id="CredentialProviderAzure-304"><a href="#CredentialProviderAzure-304"><span class="linenos">304</span></a>        <span class="bp">self</span><span class="o">.</span><span class="n">credential</span> <span class="o">=</span> <span class="n">credential</span>
</span><span id="CredentialProviderAzure-305"><a href="#CredentialProviderAzure-305"><span class="linenos">305</span></a>        <span class="nb">super</span><span class="p">()</span><span class="o">.</span><span class="fm">__init__</span><span class="p">()</span>
</span><span id="CredentialProviderAzure-306"><a href="#CredentialProviderAzure-306"><span class="linenos">306</span></a>
</span><span id="CredentialProviderAzure-307"><a href="#CredentialProviderAzure-307"><span class="linenos">307</span></a>    <span class="k">def</span><span class="w"> </span><span class="nf">retrieve_credentials_impl</span><span class="p">(</span><span class="bp">self</span><span class="p">)</span> <span class="o">-&gt;</span> <span class="n">CredentialProviderFunctionReturn</span><span class="p">:</span>
</span><span id="CredentialProviderAzure-308"><a href="#CredentialProviderAzure-308"><span class="linenos">308</span></a>        <span class="n">credential</span> <span class="o">=</span> <span class="bp">self</span><span class="o">.</span><span class="n">credential</span>
</span><span id="CredentialProviderAzure-309"><a href="#CredentialProviderAzure-309"><span class="linenos">309</span></a>        <span class="k">if</span> <span class="n">credential</span> <span class="ow">is</span> <span class="kc">None</span><span class="p">:</span>
</span><span id="CredentialProviderAzure-310"><a href="#CredentialProviderAzure-310"><span class="linenos">310</span></a>            <span class="kn">import</span><span class="w"> </span><span class="nn">azure.identity</span>
</span><span id="CredentialProviderAzure-311"><a href="#CredentialProviderAzure-311"><span class="linenos">311</span></a>
</span><span id="CredentialProviderAzure-312"><a href="#CredentialProviderAzure-312"><span class="linenos">312</span></a>            <span class="n">credential</span> <span class="o">=</span> <span class="n">azure</span><span class="o">.</span><span class="n">identity</span><span class="o">.</span><span class="n">DefaultAzureCredential</span><span class="p">()</span>
</span><span id="CredentialProviderAzure-313"><a href="#CredentialProviderAzure-313"><span class="linenos">313</span></a>        <span class="n">token</span> <span class="o">=</span> <span class="n">credential</span><span class="o">.</span><span class="n">get_token</span><span class="p">(</span><span class="o">*</span><span class="bp">self</span><span class="o">.</span><span class="n">scopes</span><span class="p">,</span> <span class="n">tenant_id</span><span class="o">=</span><span class="bp">self</span><span class="o">.</span><span class="n">tenant_id</span><span class="p">)</span>
</span><span id="CredentialProviderAzure-314"><a href="#CredentialProviderAzure-314"><span class="linenos">314</span></a>        <span class="k">return</span> <span class="p">{</span><span class="s2">"bearer_token"</span><span class="p">:</span> <span class="n">token</span><span class="o">.</span><span class="n">token</span><span class="p">},</span> <span class="n">token</span><span class="o">.</span><span class="n">expires_on</span>
</span></pre></div>


            <div class="docstring"><p>Azure credentials from <code>azure.identity</code>.</p>

<p>By default uses <code>DefaultAzureCredential()</code>, which covers environment
variables, managed identity and <code>az login</code>. Requires <code>azure-identity</code>
unless <code><a href="#CredentialProviderAzure.credential">credential</a></code> is given.</p>

<h6 id="arguments">Arguments:</h6>

<ul>
<li><strong>scopes:</strong>  Scopes to request a token for.</li>
<li><strong>tenant_id:</strong>  Azure tenant ID.</li>
<li><strong>credential:</strong>  An Azure credential object to use instead, such as
<code>azure.identity.ClientSecretCredential(...)</code>. It must have a
<code>get_token()</code> method.</li>
</ul>
</div>


                            <div id="CredentialProviderAzure.__init__" class="classattr">
                                        <input id="CredentialProviderAzure.__init__-view-source" class="view-source-toggle-state" type="checkbox" aria-hidden="true" tabindex="-1">
<div class="attr function">
            
        <span class="name">CredentialProviderAzure</span><span class="signature pdoc-code multiline">(<span class="param">    <span class="o">*</span>,</span><span class="param">    <span class="n">scopes</span><span class="p">:</span> <span class="nb">list</span><span class="p">[</span><span class="nb">str</span><span class="p">]</span> <span class="o">|</span> <span class="kc">None</span> <span class="o">=</span> <span class="kc">None</span>,</span><span class="param">    <span class="n">tenant_id</span><span class="p">:</span> <span class="nb">str</span> <span class="o">|</span> <span class="kc">None</span> <span class="o">=</span> <span class="kc">None</span>,</span><span class="param">    <span class="n">credential</span><span class="p">:</span> <span class="n">typing</span><span class="o">.</span><span class="n">Any</span> <span class="o">|</span> <span class="kc">None</span> <span class="o">=</span> <span class="kc">None</span></span>)</span>

                <label class="view-source-button" for="CredentialProviderAzure.__init__-view-source"><span>View Source</span></label>

    </div>
    <a class="headerlink" href="#CredentialProviderAzure.__init__"></a>
            <div class="pdoc-code codehilite"><pre><span></span><span id="CredentialProviderAzure.__init__-287"><a href="#CredentialProviderAzure.__init__-287"><span class="linenos">287</span></a>    <span class="k">def</span><span class="w"> </span><span class="fm">__init__</span><span class="p">(</span>
</span><span id="CredentialProviderAzure.__init__-288"><a href="#CredentialProviderAzure.__init__-288"><span class="linenos">288</span></a>        <span class="bp">self</span><span class="p">,</span>
</span><span id="CredentialProviderAzure.__init__-289"><a href="#CredentialProviderAzure.__init__-289"><span class="linenos">289</span></a>        <span class="o">*</span><span class="p">,</span>
</span><span id="CredentialProviderAzure.__init__-290"><a href="#CredentialProviderAzure.__init__-290"><span class="linenos">290</span></a>        <span class="n">scopes</span><span class="p">:</span> <span class="nb">list</span><span class="p">[</span><span class="nb">str</span><span class="p">]</span> <span class="o">|</span> <span class="kc">None</span> <span class="o">=</span> <span class="kc">None</span><span class="p">,</span>
</span><span id="CredentialProviderAzure.__init__-291"><a href="#CredentialProviderAzure.__init__-291"><span class="linenos">291</span></a>        <span class="n">tenant_id</span><span class="p">:</span> <span class="nb">str</span> <span class="o">|</span> <span class="kc">None</span> <span class="o">=</span> <span class="kc">None</span><span class="p">,</span>
</span><span id="CredentialProviderAzure.__init__-292"><a href="#CredentialProviderAzure.__init__-292"><span class="linenos">292</span></a>        <span class="n">credential</span><span class="p">:</span> <span class="n">Any</span> <span class="o">|</span> <span class="kc">None</span> <span class="o">=</span> <span class="kc">None</span><span class="p">,</span>
</span><span id="CredentialProviderAzure.__init__-293"><a href="#CredentialProviderAzure.__init__-293"><span class="linenos">293</span></a>    <span class="p">)</span> <span class="o">-&gt;</span> <span class="kc">None</span><span class="p">:</span>
</span><span id="CredentialProviderAzure.__init__-294"><a href="#CredentialProviderAzure.__init__-294"><span class="linenos">294</span></a>        <span class="k">if</span> <span class="n">credential</span> <span class="ow">is</span> <span class="kc">None</span><span class="p">:</span>
</span><span id="CredentialProviderAzure.__init__-295"><a href="#CredentialProviderAzure.__init__-295"><span class="linenos">295</span></a>            <span class="n">_require</span><span class="p">(</span><span class="s2">"azure.identity"</span><span class="p">,</span> <span class="s2">"azure-identity"</span><span class="p">,</span> <span class="s2">"CredentialProviderAzure"</span><span class="p">)</span>
</span><span id="CredentialProviderAzure.__init__-296"><a href="#CredentialProviderAzure.__init__-296"><span class="linenos">296</span></a>        <span class="k">elif</span> <span class="ow">not</span> <span class="nb">hasattr</span><span class="p">(</span><span class="n">credential</span><span class="p">,</span> <span class="s2">"get_token"</span><span class="p">):</span>
</span><span id="CredentialProviderAzure.__init__-297"><a href="#CredentialProviderAzure.__init__-297"><span class="linenos">297</span></a>            <span class="k">raise</span> <span class="ne">ValueError</span><span class="p">(</span>
</span><span id="CredentialProviderAzure.__init__-298"><a href="#CredentialProviderAzure.__init__-298"><span class="linenos">298</span></a>                <span class="sa">f</span><span class="s2">"credential </span><span class="si">{</span><span class="n">credential</span><span class="si">!r}</span><span class="s2"> does not have a get_token() method"</span>
</span><span id="CredentialProviderAzure.__init__-299"><a href="#CredentialProviderAzure.__init__-299"><span class="linenos">299</span></a>            <span class="p">)</span>
</span><span id="CredentialProviderAzure.__init__-300"><a href="#CredentialProviderAzure.__init__-300"><span class="linenos">300</span></a>        <span class="bp">self</span><span class="o">.</span><span class="n">scopes</span> <span class="o">=</span> <span class="p">(</span>
</span><span id="CredentialProviderAzure.__init__-301"><a href="#CredentialProviderAzure.__init__-301"><span class="linenos">301</span></a>            <span class="n">scopes</span> <span class="k">if</span> <span class="n">scopes</span> <span class="ow">is</span> <span class="ow">not</span> <span class="kc">None</span> <span class="k">else</span> <span class="p">[</span><span class="s2">"https://storage.azure.com/.default"</span><span class="p">]</span>
</span><span id="CredentialProviderAzure.__init__-302"><a href="#CredentialProviderAzure.__init__-302"><span class="linenos">302</span></a>        <span class="p">)</span>
</span><span id="CredentialProviderAzure.__init__-303"><a href="#CredentialProviderAzure.__init__-303"><span class="linenos">303</span></a>        <span class="bp">self</span><span class="o">.</span><span class="n">tenant_id</span> <span class="o">=</span> <span class="n">tenant_id</span>
</span><span id="CredentialProviderAzure.__init__-304"><a href="#CredentialProviderAzure.__init__-304"><span class="linenos">304</span></a>        <span class="bp">self</span><span class="o">.</span><span class="n">credential</span> <span class="o">=</span> <span class="n">credential</span>
</span><span id="CredentialProviderAzure.__init__-305"><a href="#CredentialProviderAzure.__init__-305"><span class="linenos">305</span></a>        <span class="nb">super</span><span class="p">()</span><span class="o">.</span><span class="fm">__init__</span><span class="p">()</span>
</span></pre></div>


    

                            </div>
                            <div id="CredentialProviderAzure.credential" class="classattr">
                                <div class="attr variable">
            <span class="name">credential</span>

        
    </div>
    <a class="headerlink" href="#CredentialProviderAzure.credential"></a>
    
    

                            </div>
                            <div id="CredentialProviderAzure.retrieve_credentials_impl" class="classattr">
                                        <input id="CredentialProviderAzure.retrieve_credentials_impl-view-source" class="view-source-toggle-state" type="checkbox" aria-hidden="true" tabindex="-1">
<div class="attr function">
            
        <span class="def">def</span>
        <span class="name">retrieve_credentials_impl</span><span class="signature pdoc-code condensed">(<span class="param"><span class="bp">self</span></span><span class="return-annotation">) -> <span class="nb">tuple</span><span class="p">[</span><span class="nb">dict</span><span class="p">[</span><span class="nb">str</span><span class="p">,</span> <span class="nb">str</span><span class="p">],</span> <span class="n">typing</span><span class="o">.</span><span class="n">Optional</span><span class="p">[</span><span class="nb">int</span><span class="p">]]</span>:</span></span>

                <label class="view-source-button" for="CredentialProviderAzure.retrieve_credentials_impl-view-source"><span>View Source</span></label>

    </div>
    <a class="headerlink" href="#CredentialProviderAzure.retrieve_credentials_impl"></a>
            <div class="pdoc-code codehilite"><pre><span></span><span id="CredentialProviderAzure.retrieve_credentials_impl-307"><a href="#CredentialProviderAzure.retrieve_credentials_impl-307"><span class="linenos">307</span></a>    <span class="k">def</span><span class="w"> </span><span class="nf">retrieve_credentials_impl</span><span class="p">(</span><span class="bp">self</span><span class="p">)</span> <span class="o">-&gt;</span> <span class="n">CredentialProviderFunctionReturn</span><span class="p">:</span>
</span><span id="CredentialProviderAzure.retrieve_credentials_impl-308"><a href="#CredentialProviderAzure.retrieve_credentials_impl-308"><span class="linenos">308</span></a>        <span class="n">credential</span> <span class="o">=</span> <span class="bp">self</span><span class="o">.</span><span class="n">credential</span>
</span><span id="CredentialProviderAzure.retrieve_credentials_impl-309"><a href="#CredentialProviderAzure.retrieve_credentials_impl-309"><span class="linenos">309</span></a>        <span class="k">if</span> <span class="n">credential</span> <span class="ow">is</span> <span class="kc">None</span><span class="p">:</span>
</span><span id="CredentialProviderAzure.retrieve_credentials_impl-310"><a href="#CredentialProviderAzure.retrieve_credentials_impl-310"><span class="linenos">310</span></a>            <span class="kn">import</span><span class="w"> </span><span class="nn">azure.identity</span>
</span><span id="CredentialProviderAzure.retrieve_credentials_impl-311"><a href="#CredentialProviderAzure.retrieve_credentials_impl-311"><span class="linenos">311</span></a>
</span><span id="CredentialProviderAzure.retrieve_credentials_impl-312"><a href="#CredentialProviderAzure.retrieve_credentials_impl-312"><span class="linenos">312</span></a>            <span class="n">credential</span> <span class="o">=</span> <span class="n">azure</span><span class="o">.</span><span class="n">identity</span><span class="o">.</span><span class="n">DefaultAzureCredential</span><span class="p">()</span>
</span><span id="CredentialProviderAzure.retrieve_credentials_impl-313"><a href="#CredentialProviderAzure.retrieve_credentials_impl-313"><span class="linenos">313</span></a>        <span class="n">token</span> <span class="o">=</span> <span class="n">credential</span><span class="o">.</span><span class="n">get_token</span><span class="p">(</span><span class="o">*</span><span class="bp">self</span><span class="o">.</span><span class="n">scopes</span><span class="p">,</span> <span class="n">tenant_id</span><span class="o">=</span><span class="bp">self</span><span class="o">.</span><span class="n">tenant_id</span><span class="p">)</span>
</span><span id="CredentialProviderAzure.retrieve_credentials_impl-314"><a href="#CredentialProviderAzure.retrieve_credentials_impl-314"><span class="linenos">314</span></a>        <span class="k">return</span> <span class="p">{</span><span class="s2">"bearer_token"</span><span class="p">:</span> <span class="n">token</span><span class="o">.</span><span class="n">token</span><span class="p">},</span> <span class="n">token</span><span class="o">.</span><span class="n">expires_on</span>
</span></pre></div>


            <div class="docstring"><p>Fetch new credentials, bypassing the cache.</p>
</div>


                            </div>
                            <div id="CredentialProviderAzure.scopes" class="classattr">
                                <div class="attr variable">
            <span class="name">scopes</span>

        
    </div>
    <a class="headerlink" href="#CredentialProviderAzure.scopes"></a>
    
    

                            </div>
                            <div id="CredentialProviderAzure.tenant_id" class="classattr">
                                <div class="attr variable">
            <span class="name">tenant_id</span>

        
    </div>
    <a class="headerlink" href="#CredentialProviderAzure.tenant_id"></a>
    
    

                            </div>
                            <div class="inherited">
                                <h5>Inherited Members</h5>
                                <dl>
                                    <div><dt><a href="#CredentialProvider">CredentialProvider</a></dt>
                                <dd id="CredentialProviderAzure.clear_cached_credentials" class="function"><a href="#CredentialProvider.clear_cached_credentials">clear_cached_credentials</a></dd>

            </div>
                                </dl>
                            </div>
                </section>
</main>

## CredentialProviderFunction

<main class="pdoc">
<section id="CredentialProviderFunction">
                    <div class="attr variable">
            <span class="name">CredentialProviderFunction</span>        =
<input id="CredentialProviderFunction-view-value" class="view-value-toggle-state" type="checkbox" aria-hidden="true" tabindex="-1">
            <label class="view-value-button pdoc-button" for="CredentialProviderFunction-view-value"></label><span class="default_value">typing.Union[collections.abc.Callable[[], tuple[dict[str, str], typing.Optional[int]]], ForwardRef(&#39;CredentialProvider&#39;)]</span>

        
    </div>
    <a class="headerlink" href="#CredentialProviderFunction"></a>
    
            <div class="docstring"><p>Anything accepted as <code>credential_provider</code>.</p>
</div>


                </section>
</main>

## CredentialProviderFunctionReturn

<main class="pdoc">
<section id="CredentialProviderFunctionReturn">
                    <div class="attr variable">
            <span class="name">CredentialProviderFunctionReturn</span>        =
<span class="default_value">tuple[dict[str, str], typing.Optional[int]]</span>

        
    </div>
    <a class="headerlink" href="#CredentialProviderFunctionReturn"></a>
    
            <div class="docstring"><p>The <code>(credentials, expiry)</code> pair a credential provider returns.</p>
</div>


                </section>
</main>

## CredentialProviderGCP

<main class="pdoc">
<section id="CredentialProviderGCP">
                            <input id="CredentialProviderGCP-view-source" class="view-source-toggle-state" type="checkbox" aria-hidden="true" tabindex="-1">
<div class="attr class">
            
    <span class="def">class</span>
    <span class="name">CredentialProviderGCP</span><wbr>(<span class="base"><a href="#CredentialProvider">CredentialProvider</a></span>):

                <label class="view-source-button" for="CredentialProviderGCP-view-source"><span>View Source</span></label>

    </div>
    <a class="headerlink" href="#CredentialProviderGCP"></a>
            <div class="pdoc-code codehilite"><pre><span></span><span id="CredentialProviderGCP-224"><a href="#CredentialProviderGCP-224"><span class="linenos">224</span></a><span class="k">class</span><span class="w"> </span><span class="nc">CredentialProviderGCP</span><span class="p">(</span><span class="n">CredentialProvider</span><span class="p">):</span>
</span><span id="CredentialProviderGCP-225"><a href="#CredentialProviderGCP-225"><span class="linenos">225</span></a><span class="w">    </span><span class="sd">"""Google Cloud credentials from ``google.auth.default()``.</span>
</span><span id="CredentialProviderGCP-226"><a href="#CredentialProviderGCP-226"><span class="linenos">226</span></a>
</span><span id="CredentialProviderGCP-227"><a href="#CredentialProviderGCP-227"><span class="linenos">227</span></a><span class="sd">    Covers application default credentials, ``gcloud auth``, service account</span>
</span><span id="CredentialProviderGCP-228"><a href="#CredentialProviderGCP-228"><span class="linenos">228</span></a><span class="sd">    files (``GOOGLE_APPLICATION_CREDENTIALS``) and workload identity.</span>
</span><span id="CredentialProviderGCP-229"><a href="#CredentialProviderGCP-229"><span class="linenos">229</span></a><span class="sd">    Requires ``google-auth``. The arguments are passed to</span>
</span><span id="CredentialProviderGCP-230"><a href="#CredentialProviderGCP-230"><span class="linenos">230</span></a><span class="sd">    ``google.auth.default()``.</span>
</span><span id="CredentialProviderGCP-231"><a href="#CredentialProviderGCP-231"><span class="linenos">231</span></a><span class="sd">    """</span>
</span><span id="CredentialProviderGCP-232"><a href="#CredentialProviderGCP-232"><span class="linenos">232</span></a>
</span><span id="CredentialProviderGCP-233"><a href="#CredentialProviderGCP-233"><span class="linenos">233</span></a>    <span class="k">def</span><span class="w"> </span><span class="fm">__init__</span><span class="p">(</span>
</span><span id="CredentialProviderGCP-234"><a href="#CredentialProviderGCP-234"><span class="linenos">234</span></a>        <span class="bp">self</span><span class="p">,</span>
</span><span id="CredentialProviderGCP-235"><a href="#CredentialProviderGCP-235"><span class="linenos">235</span></a>        <span class="o">*</span><span class="p">,</span>
</span><span id="CredentialProviderGCP-236"><a href="#CredentialProviderGCP-236"><span class="linenos">236</span></a>        <span class="n">scopes</span><span class="p">:</span> <span class="n">Any</span> <span class="o">|</span> <span class="kc">None</span> <span class="o">=</span> <span class="kc">None</span><span class="p">,</span>
</span><span id="CredentialProviderGCP-237"><a href="#CredentialProviderGCP-237"><span class="linenos">237</span></a>        <span class="n">request</span><span class="p">:</span> <span class="n">Any</span> <span class="o">|</span> <span class="kc">None</span> <span class="o">=</span> <span class="kc">None</span><span class="p">,</span>
</span><span id="CredentialProviderGCP-238"><a href="#CredentialProviderGCP-238"><span class="linenos">238</span></a>        <span class="n">quota_project_id</span><span class="p">:</span> <span class="n">Any</span> <span class="o">|</span> <span class="kc">None</span> <span class="o">=</span> <span class="kc">None</span><span class="p">,</span>
</span><span id="CredentialProviderGCP-239"><a href="#CredentialProviderGCP-239"><span class="linenos">239</span></a>        <span class="n">default_scopes</span><span class="p">:</span> <span class="n">Any</span> <span class="o">|</span> <span class="kc">None</span> <span class="o">=</span> <span class="kc">None</span><span class="p">,</span>
</span><span id="CredentialProviderGCP-240"><a href="#CredentialProviderGCP-240"><span class="linenos">240</span></a>    <span class="p">)</span> <span class="o">-&gt;</span> <span class="kc">None</span><span class="p">:</span>
</span><span id="CredentialProviderGCP-241"><a href="#CredentialProviderGCP-241"><span class="linenos">241</span></a>        <span class="n">_require</span><span class="p">(</span><span class="s2">"google.auth"</span><span class="p">,</span> <span class="s2">"google-auth"</span><span class="p">,</span> <span class="s2">"CredentialProviderGCP"</span><span class="p">)</span>
</span><span id="CredentialProviderGCP-242"><a href="#CredentialProviderGCP-242"><span class="linenos">242</span></a>        <span class="bp">self</span><span class="o">.</span><span class="n">scopes</span> <span class="o">=</span> <span class="p">(</span>
</span><span id="CredentialProviderGCP-243"><a href="#CredentialProviderGCP-243"><span class="linenos">243</span></a>            <span class="n">scopes</span>
</span><span id="CredentialProviderGCP-244"><a href="#CredentialProviderGCP-244"><span class="linenos">244</span></a>            <span class="k">if</span> <span class="n">scopes</span> <span class="ow">is</span> <span class="ow">not</span> <span class="kc">None</span>
</span><span id="CredentialProviderGCP-245"><a href="#CredentialProviderGCP-245"><span class="linenos">245</span></a>            <span class="k">else</span> <span class="p">[</span><span class="s2">"https://www.googleapis.com/auth/cloud-platform"</span><span class="p">]</span>
</span><span id="CredentialProviderGCP-246"><a href="#CredentialProviderGCP-246"><span class="linenos">246</span></a>        <span class="p">)</span>
</span><span id="CredentialProviderGCP-247"><a href="#CredentialProviderGCP-247"><span class="linenos">247</span></a>        <span class="bp">self</span><span class="o">.</span><span class="n">request</span> <span class="o">=</span> <span class="n">request</span>
</span><span id="CredentialProviderGCP-248"><a href="#CredentialProviderGCP-248"><span class="linenos">248</span></a>        <span class="bp">self</span><span class="o">.</span><span class="n">quota_project_id</span> <span class="o">=</span> <span class="n">quota_project_id</span>
</span><span id="CredentialProviderGCP-249"><a href="#CredentialProviderGCP-249"><span class="linenos">249</span></a>        <span class="bp">self</span><span class="o">.</span><span class="n">default_scopes</span> <span class="o">=</span> <span class="n">default_scopes</span>
</span><span id="CredentialProviderGCP-250"><a href="#CredentialProviderGCP-250"><span class="linenos">250</span></a>        <span class="nb">super</span><span class="p">()</span><span class="o">.</span><span class="fm">__init__</span><span class="p">()</span>
</span><span id="CredentialProviderGCP-251"><a href="#CredentialProviderGCP-251"><span class="linenos">251</span></a>
</span><span id="CredentialProviderGCP-252"><a href="#CredentialProviderGCP-252"><span class="linenos">252</span></a>    <span class="k">def</span><span class="w"> </span><span class="nf">retrieve_credentials_impl</span><span class="p">(</span><span class="bp">self</span><span class="p">)</span> <span class="o">-&gt;</span> <span class="n">CredentialProviderFunctionReturn</span><span class="p">:</span>
</span><span id="CredentialProviderGCP-253"><a href="#CredentialProviderGCP-253"><span class="linenos">253</span></a>        <span class="kn">import</span><span class="w"> </span><span class="nn">google.auth</span>
</span><span id="CredentialProviderGCP-254"><a href="#CredentialProviderGCP-254"><span class="linenos">254</span></a>        <span class="kn">import</span><span class="w"> </span><span class="nn">google.auth.transport.requests</span>
</span><span id="CredentialProviderGCP-255"><a href="#CredentialProviderGCP-255"><span class="linenos">255</span></a>
</span><span id="CredentialProviderGCP-256"><a href="#CredentialProviderGCP-256"><span class="linenos">256</span></a>        <span class="n">creds</span><span class="p">,</span> <span class="n">_project_id</span> <span class="o">=</span> <span class="n">google</span><span class="o">.</span><span class="n">auth</span><span class="o">.</span><span class="n">default</span><span class="p">(</span>
</span><span id="CredentialProviderGCP-257"><a href="#CredentialProviderGCP-257"><span class="linenos">257</span></a>            <span class="n">scopes</span><span class="o">=</span><span class="bp">self</span><span class="o">.</span><span class="n">scopes</span><span class="p">,</span>
</span><span id="CredentialProviderGCP-258"><a href="#CredentialProviderGCP-258"><span class="linenos">258</span></a>            <span class="n">request</span><span class="o">=</span><span class="bp">self</span><span class="o">.</span><span class="n">request</span><span class="p">,</span>
</span><span id="CredentialProviderGCP-259"><a href="#CredentialProviderGCP-259"><span class="linenos">259</span></a>            <span class="n">quota_project_id</span><span class="o">=</span><span class="bp">self</span><span class="o">.</span><span class="n">quota_project_id</span><span class="p">,</span>
</span><span id="CredentialProviderGCP-260"><a href="#CredentialProviderGCP-260"><span class="linenos">260</span></a>            <span class="n">default_scopes</span><span class="o">=</span><span class="bp">self</span><span class="o">.</span><span class="n">default_scopes</span><span class="p">,</span>
</span><span id="CredentialProviderGCP-261"><a href="#CredentialProviderGCP-261"><span class="linenos">261</span></a>        <span class="p">)</span>
</span><span id="CredentialProviderGCP-262"><a href="#CredentialProviderGCP-262"><span class="linenos">262</span></a>        <span class="n">creds</span><span class="o">.</span><span class="n">refresh</span><span class="p">(</span><span class="n">google</span><span class="o">.</span><span class="n">auth</span><span class="o">.</span><span class="n">transport</span><span class="o">.</span><span class="n">requests</span><span class="o">.</span><span class="n">Request</span><span class="p">())</span>
</span><span id="CredentialProviderGCP-263"><a href="#CredentialProviderGCP-263"><span class="linenos">263</span></a>        <span class="n">expiry</span> <span class="o">=</span> <span class="n">creds</span><span class="o">.</span><span class="n">expiry</span>
</span><span id="CredentialProviderGCP-264"><a href="#CredentialProviderGCP-264"><span class="linenos">264</span></a>        <span class="k">if</span> <span class="n">expiry</span> <span class="ow">is</span> <span class="ow">not</span> <span class="kc">None</span> <span class="ow">and</span> <span class="n">expiry</span><span class="o">.</span><span class="n">tzinfo</span> <span class="ow">is</span> <span class="kc">None</span><span class="p">:</span>
</span><span id="CredentialProviderGCP-265"><a href="#CredentialProviderGCP-265"><span class="linenos">265</span></a>            <span class="c1"># google-auth reports naive UTC times.</span>
</span><span id="CredentialProviderGCP-266"><a href="#CredentialProviderGCP-266"><span class="linenos">266</span></a>            <span class="n">expiry</span> <span class="o">=</span> <span class="n">expiry</span><span class="o">.</span><span class="n">replace</span><span class="p">(</span><span class="n">tzinfo</span><span class="o">=</span><span class="n">timezone</span><span class="o">.</span><span class="n">utc</span><span class="p">)</span>
</span><span id="CredentialProviderGCP-267"><a href="#CredentialProviderGCP-267"><span class="linenos">267</span></a>        <span class="k">return</span> <span class="p">{</span><span class="s2">"bearer_token"</span><span class="p">:</span> <span class="n">creds</span><span class="o">.</span><span class="n">token</span><span class="p">},</span> <span class="p">(</span>
</span><span id="CredentialProviderGCP-268"><a href="#CredentialProviderGCP-268"><span class="linenos">268</span></a>            <span class="nb">int</span><span class="p">(</span><span class="n">expiry</span><span class="o">.</span><span class="n">timestamp</span><span class="p">())</span> <span class="k">if</span> <span class="n">expiry</span> <span class="ow">is</span> <span class="ow">not</span> <span class="kc">None</span> <span class="k">else</span> <span class="kc">None</span>
</span><span id="CredentialProviderGCP-269"><a href="#CredentialProviderGCP-269"><span class="linenos">269</span></a>        <span class="p">)</span>
</span></pre></div>


            <div class="docstring"><p>Google Cloud credentials from <code>google.auth.default()</code>.</p>

<p>Covers application default credentials, <code>gcloud auth</code>, service account
files (<code>GOOGLE_APPLICATION_CREDENTIALS</code>) and workload identity.
Requires <code>google-auth</code>. The arguments are passed to
<code>google.auth.default()</code>.</p>
</div>


                            <div id="CredentialProviderGCP.__init__" class="classattr">
                                        <input id="CredentialProviderGCP.__init__-view-source" class="view-source-toggle-state" type="checkbox" aria-hidden="true" tabindex="-1">
<div class="attr function">
            
        <span class="name">CredentialProviderGCP</span><span class="signature pdoc-code multiline">(<span class="param">    <span class="o">*</span>,</span><span class="param">    <span class="n">scopes</span><span class="p">:</span> <span class="n">typing</span><span class="o">.</span><span class="n">Any</span> <span class="o">|</span> <span class="kc">None</span> <span class="o">=</span> <span class="kc">None</span>,</span><span class="param">    <span class="n">request</span><span class="p">:</span> <span class="n">typing</span><span class="o">.</span><span class="n">Any</span> <span class="o">|</span> <span class="kc">None</span> <span class="o">=</span> <span class="kc">None</span>,</span><span class="param">    <span class="n">quota_project_id</span><span class="p">:</span> <span class="n">typing</span><span class="o">.</span><span class="n">Any</span> <span class="o">|</span> <span class="kc">None</span> <span class="o">=</span> <span class="kc">None</span>,</span><span class="param">    <span class="n">default_scopes</span><span class="p">:</span> <span class="n">typing</span><span class="o">.</span><span class="n">Any</span> <span class="o">|</span> <span class="kc">None</span> <span class="o">=</span> <span class="kc">None</span></span>)</span>

                <label class="view-source-button" for="CredentialProviderGCP.__init__-view-source"><span>View Source</span></label>

    </div>
    <a class="headerlink" href="#CredentialProviderGCP.__init__"></a>
            <div class="pdoc-code codehilite"><pre><span></span><span id="CredentialProviderGCP.__init__-233"><a href="#CredentialProviderGCP.__init__-233"><span class="linenos">233</span></a>    <span class="k">def</span><span class="w"> </span><span class="fm">__init__</span><span class="p">(</span>
</span><span id="CredentialProviderGCP.__init__-234"><a href="#CredentialProviderGCP.__init__-234"><span class="linenos">234</span></a>        <span class="bp">self</span><span class="p">,</span>
</span><span id="CredentialProviderGCP.__init__-235"><a href="#CredentialProviderGCP.__init__-235"><span class="linenos">235</span></a>        <span class="o">*</span><span class="p">,</span>
</span><span id="CredentialProviderGCP.__init__-236"><a href="#CredentialProviderGCP.__init__-236"><span class="linenos">236</span></a>        <span class="n">scopes</span><span class="p">:</span> <span class="n">Any</span> <span class="o">|</span> <span class="kc">None</span> <span class="o">=</span> <span class="kc">None</span><span class="p">,</span>
</span><span id="CredentialProviderGCP.__init__-237"><a href="#CredentialProviderGCP.__init__-237"><span class="linenos">237</span></a>        <span class="n">request</span><span class="p">:</span> <span class="n">Any</span> <span class="o">|</span> <span class="kc">None</span> <span class="o">=</span> <span class="kc">None</span><span class="p">,</span>
</span><span id="CredentialProviderGCP.__init__-238"><a href="#CredentialProviderGCP.__init__-238"><span class="linenos">238</span></a>        <span class="n">quota_project_id</span><span class="p">:</span> <span class="n">Any</span> <span class="o">|</span> <span class="kc">None</span> <span class="o">=</span> <span class="kc">None</span><span class="p">,</span>
</span><span id="CredentialProviderGCP.__init__-239"><a href="#CredentialProviderGCP.__init__-239"><span class="linenos">239</span></a>        <span class="n">default_scopes</span><span class="p">:</span> <span class="n">Any</span> <span class="o">|</span> <span class="kc">None</span> <span class="o">=</span> <span class="kc">None</span><span class="p">,</span>
</span><span id="CredentialProviderGCP.__init__-240"><a href="#CredentialProviderGCP.__init__-240"><span class="linenos">240</span></a>    <span class="p">)</span> <span class="o">-&gt;</span> <span class="kc">None</span><span class="p">:</span>
</span><span id="CredentialProviderGCP.__init__-241"><a href="#CredentialProviderGCP.__init__-241"><span class="linenos">241</span></a>        <span class="n">_require</span><span class="p">(</span><span class="s2">"google.auth"</span><span class="p">,</span> <span class="s2">"google-auth"</span><span class="p">,</span> <span class="s2">"CredentialProviderGCP"</span><span class="p">)</span>
</span><span id="CredentialProviderGCP.__init__-242"><a href="#CredentialProviderGCP.__init__-242"><span class="linenos">242</span></a>        <span class="bp">self</span><span class="o">.</span><span class="n">scopes</span> <span class="o">=</span> <span class="p">(</span>
</span><span id="CredentialProviderGCP.__init__-243"><a href="#CredentialProviderGCP.__init__-243"><span class="linenos">243</span></a>            <span class="n">scopes</span>
</span><span id="CredentialProviderGCP.__init__-244"><a href="#CredentialProviderGCP.__init__-244"><span class="linenos">244</span></a>            <span class="k">if</span> <span class="n">scopes</span> <span class="ow">is</span> <span class="ow">not</span> <span class="kc">None</span>
</span><span id="CredentialProviderGCP.__init__-245"><a href="#CredentialProviderGCP.__init__-245"><span class="linenos">245</span></a>            <span class="k">else</span> <span class="p">[</span><span class="s2">"https://www.googleapis.com/auth/cloud-platform"</span><span class="p">]</span>
</span><span id="CredentialProviderGCP.__init__-246"><a href="#CredentialProviderGCP.__init__-246"><span class="linenos">246</span></a>        <span class="p">)</span>
</span><span id="CredentialProviderGCP.__init__-247"><a href="#CredentialProviderGCP.__init__-247"><span class="linenos">247</span></a>        <span class="bp">self</span><span class="o">.</span><span class="n">request</span> <span class="o">=</span> <span class="n">request</span>
</span><span id="CredentialProviderGCP.__init__-248"><a href="#CredentialProviderGCP.__init__-248"><span class="linenos">248</span></a>        <span class="bp">self</span><span class="o">.</span><span class="n">quota_project_id</span> <span class="o">=</span> <span class="n">quota_project_id</span>
</span><span id="CredentialProviderGCP.__init__-249"><a href="#CredentialProviderGCP.__init__-249"><span class="linenos">249</span></a>        <span class="bp">self</span><span class="o">.</span><span class="n">default_scopes</span> <span class="o">=</span> <span class="n">default_scopes</span>
</span><span id="CredentialProviderGCP.__init__-250"><a href="#CredentialProviderGCP.__init__-250"><span class="linenos">250</span></a>        <span class="nb">super</span><span class="p">()</span><span class="o">.</span><span class="fm">__init__</span><span class="p">()</span>
</span></pre></div>


    

                            </div>
                            <div id="CredentialProviderGCP.default_scopes" class="classattr">
                                <div class="attr variable">
            <span class="name">default_scopes</span>

        
    </div>
    <a class="headerlink" href="#CredentialProviderGCP.default_scopes"></a>
    
    

                            </div>
                            <div id="CredentialProviderGCP.quota_project_id" class="classattr">
                                <div class="attr variable">
            <span class="name">quota_project_id</span>

        
    </div>
    <a class="headerlink" href="#CredentialProviderGCP.quota_project_id"></a>
    
    

                            </div>
                            <div id="CredentialProviderGCP.request" class="classattr">
                                <div class="attr variable">
            <span class="name">request</span>

        
    </div>
    <a class="headerlink" href="#CredentialProviderGCP.request"></a>
    
    

                            </div>
                            <div id="CredentialProviderGCP.retrieve_credentials_impl" class="classattr">
                                        <input id="CredentialProviderGCP.retrieve_credentials_impl-view-source" class="view-source-toggle-state" type="checkbox" aria-hidden="true" tabindex="-1">
<div class="attr function">
            
        <span class="def">def</span>
        <span class="name">retrieve_credentials_impl</span><span class="signature pdoc-code condensed">(<span class="param"><span class="bp">self</span></span><span class="return-annotation">) -> <span class="nb">tuple</span><span class="p">[</span><span class="nb">dict</span><span class="p">[</span><span class="nb">str</span><span class="p">,</span> <span class="nb">str</span><span class="p">],</span> <span class="n">typing</span><span class="o">.</span><span class="n">Optional</span><span class="p">[</span><span class="nb">int</span><span class="p">]]</span>:</span></span>

                <label class="view-source-button" for="CredentialProviderGCP.retrieve_credentials_impl-view-source"><span>View Source</span></label>

    </div>
    <a class="headerlink" href="#CredentialProviderGCP.retrieve_credentials_impl"></a>
            <div class="pdoc-code codehilite"><pre><span></span><span id="CredentialProviderGCP.retrieve_credentials_impl-252"><a href="#CredentialProviderGCP.retrieve_credentials_impl-252"><span class="linenos">252</span></a>    <span class="k">def</span><span class="w"> </span><span class="nf">retrieve_credentials_impl</span><span class="p">(</span><span class="bp">self</span><span class="p">)</span> <span class="o">-&gt;</span> <span class="n">CredentialProviderFunctionReturn</span><span class="p">:</span>
</span><span id="CredentialProviderGCP.retrieve_credentials_impl-253"><a href="#CredentialProviderGCP.retrieve_credentials_impl-253"><span class="linenos">253</span></a>        <span class="kn">import</span><span class="w"> </span><span class="nn">google.auth</span>
</span><span id="CredentialProviderGCP.retrieve_credentials_impl-254"><a href="#CredentialProviderGCP.retrieve_credentials_impl-254"><span class="linenos">254</span></a>        <span class="kn">import</span><span class="w"> </span><span class="nn">google.auth.transport.requests</span>
</span><span id="CredentialProviderGCP.retrieve_credentials_impl-255"><a href="#CredentialProviderGCP.retrieve_credentials_impl-255"><span class="linenos">255</span></a>
</span><span id="CredentialProviderGCP.retrieve_credentials_impl-256"><a href="#CredentialProviderGCP.retrieve_credentials_impl-256"><span class="linenos">256</span></a>        <span class="n">creds</span><span class="p">,</span> <span class="n">_project_id</span> <span class="o">=</span> <span class="n">google</span><span class="o">.</span><span class="n">auth</span><span class="o">.</span><span class="n">default</span><span class="p">(</span>
</span><span id="CredentialProviderGCP.retrieve_credentials_impl-257"><a href="#CredentialProviderGCP.retrieve_credentials_impl-257"><span class="linenos">257</span></a>            <span class="n">scopes</span><span class="o">=</span><span class="bp">self</span><span class="o">.</span><span class="n">scopes</span><span class="p">,</span>
</span><span id="CredentialProviderGCP.retrieve_credentials_impl-258"><a href="#CredentialProviderGCP.retrieve_credentials_impl-258"><span class="linenos">258</span></a>            <span class="n">request</span><span class="o">=</span><span class="bp">self</span><span class="o">.</span><span class="n">request</span><span class="p">,</span>
</span><span id="CredentialProviderGCP.retrieve_credentials_impl-259"><a href="#CredentialProviderGCP.retrieve_credentials_impl-259"><span class="linenos">259</span></a>            <span class="n">quota_project_id</span><span class="o">=</span><span class="bp">self</span><span class="o">.</span><span class="n">quota_project_id</span><span class="p">,</span>
</span><span id="CredentialProviderGCP.retrieve_credentials_impl-260"><a href="#CredentialProviderGCP.retrieve_credentials_impl-260"><span class="linenos">260</span></a>            <span class="n">default_scopes</span><span class="o">=</span><span class="bp">self</span><span class="o">.</span><span class="n">default_scopes</span><span class="p">,</span>
</span><span id="CredentialProviderGCP.retrieve_credentials_impl-261"><a href="#CredentialProviderGCP.retrieve_credentials_impl-261"><span class="linenos">261</span></a>        <span class="p">)</span>
</span><span id="CredentialProviderGCP.retrieve_credentials_impl-262"><a href="#CredentialProviderGCP.retrieve_credentials_impl-262"><span class="linenos">262</span></a>        <span class="n">creds</span><span class="o">.</span><span class="n">refresh</span><span class="p">(</span><span class="n">google</span><span class="o">.</span><span class="n">auth</span><span class="o">.</span><span class="n">transport</span><span class="o">.</span><span class="n">requests</span><span class="o">.</span><span class="n">Request</span><span class="p">())</span>
</span><span id="CredentialProviderGCP.retrieve_credentials_impl-263"><a href="#CredentialProviderGCP.retrieve_credentials_impl-263"><span class="linenos">263</span></a>        <span class="n">expiry</span> <span class="o">=</span> <span class="n">creds</span><span class="o">.</span><span class="n">expiry</span>
</span><span id="CredentialProviderGCP.retrieve_credentials_impl-264"><a href="#CredentialProviderGCP.retrieve_credentials_impl-264"><span class="linenos">264</span></a>        <span class="k">if</span> <span class="n">expiry</span> <span class="ow">is</span> <span class="ow">not</span> <span class="kc">None</span> <span class="ow">and</span> <span class="n">expiry</span><span class="o">.</span><span class="n">tzinfo</span> <span class="ow">is</span> <span class="kc">None</span><span class="p">:</span>
</span><span id="CredentialProviderGCP.retrieve_credentials_impl-265"><a href="#CredentialProviderGCP.retrieve_credentials_impl-265"><span class="linenos">265</span></a>            <span class="c1"># google-auth reports naive UTC times.</span>
</span><span id="CredentialProviderGCP.retrieve_credentials_impl-266"><a href="#CredentialProviderGCP.retrieve_credentials_impl-266"><span class="linenos">266</span></a>            <span class="n">expiry</span> <span class="o">=</span> <span class="n">expiry</span><span class="o">.</span><span class="n">replace</span><span class="p">(</span><span class="n">tzinfo</span><span class="o">=</span><span class="n">timezone</span><span class="o">.</span><span class="n">utc</span><span class="p">)</span>
</span><span id="CredentialProviderGCP.retrieve_credentials_impl-267"><a href="#CredentialProviderGCP.retrieve_credentials_impl-267"><span class="linenos">267</span></a>        <span class="k">return</span> <span class="p">{</span><span class="s2">"bearer_token"</span><span class="p">:</span> <span class="n">creds</span><span class="o">.</span><span class="n">token</span><span class="p">},</span> <span class="p">(</span>
</span><span id="CredentialProviderGCP.retrieve_credentials_impl-268"><a href="#CredentialProviderGCP.retrieve_credentials_impl-268"><span class="linenos">268</span></a>            <span class="nb">int</span><span class="p">(</span><span class="n">expiry</span><span class="o">.</span><span class="n">timestamp</span><span class="p">())</span> <span class="k">if</span> <span class="n">expiry</span> <span class="ow">is</span> <span class="ow">not</span> <span class="kc">None</span> <span class="k">else</span> <span class="kc">None</span>
</span><span id="CredentialProviderGCP.retrieve_credentials_impl-269"><a href="#CredentialProviderGCP.retrieve_credentials_impl-269"><span class="linenos">269</span></a>        <span class="p">)</span>
</span></pre></div>


            <div class="docstring"><p>Fetch new credentials, bypassing the cache.</p>
</div>


                            </div>
                            <div id="CredentialProviderGCP.scopes" class="classattr">
                                <div class="attr variable">
            <span class="name">scopes</span>

        
    </div>
    <a class="headerlink" href="#CredentialProviderGCP.scopes"></a>
    
    

                            </div>
                            <div class="inherited">
                                <h5>Inherited Members</h5>
                                <dl>
                                    <div><dt><a href="#CredentialProvider">CredentialProvider</a></dt>
                                <dd id="CredentialProviderGCP.clear_cached_credentials" class="function"><a href="#CredentialProvider.clear_cached_credentials">clear_cached_credentials</a></dd>

            </div>
                                </dl>
                            </div>
                </section>
</main>

<style>pre{line-height:125%;}span.linenos{color:inherit; background-color:transparent; padding-left:5px; padding-right:20px;}.pdoc-code .hll{background-color:#ffffcc}.pdoc-code{background:#f8f8f8;}.pdoc-code .c{color:#3D7B7B; font-style:italic}.pdoc-code .err{border:1px solid #FF0000}.pdoc-code .k{color:#008000; font-weight:bold}.pdoc-code .o{color:#666666}.pdoc-code .ch{color:#3D7B7B; font-style:italic}.pdoc-code .cm{color:#3D7B7B; font-style:italic}.pdoc-code .cp{color:#9C6500}.pdoc-code .cpf{color:#3D7B7B; font-style:italic}.pdoc-code .c1{color:#3D7B7B; font-style:italic}.pdoc-code .cs{color:#3D7B7B; font-style:italic}.pdoc-code .gd{color:#A00000}.pdoc-code .ge{font-style:italic}.pdoc-code .gr{color:#E40000}.pdoc-code .gh{color:#000080; font-weight:bold}.pdoc-code .gi{color:#008400}.pdoc-code .go{color:#717171}.pdoc-code .gp{color:#000080; font-weight:bold}.pdoc-code .gs{font-weight:bold}.pdoc-code .gu{color:#800080; font-weight:bold}.pdoc-code .gt{color:#0044DD}.pdoc-code .kc{color:#008000; font-weight:bold}.pdoc-code .kd{color:#008000; font-weight:bold}.pdoc-code .kn{color:#008000; font-weight:bold}.pdoc-code .kp{color:#008000}.pdoc-code .kr{color:#008000; font-weight:bold}.pdoc-code .kt{color:#B00040}.pdoc-code .m{color:#666666}.pdoc-code .s{color:#BA2121}.pdoc-code .na{color:#687822}.pdoc-code .nb{color:#008000}.pdoc-code .nc{color:#0000FF; font-weight:bold}.pdoc-code .no{color:#880000}.pdoc-code .nd{color:#AA22FF}.pdoc-code .ni{color:#717171; font-weight:bold}.pdoc-code .ne{color:#CB3F38; font-weight:bold}.pdoc-code .nf{color:#0000FF}.pdoc-code .nl{color:#767600}.pdoc-code .nn{color:#0000FF; font-weight:bold}.pdoc-code .nt{color:#008000; font-weight:bold}.pdoc-code .nv{color:#19177C}.pdoc-code .ow{color:#AA22FF; font-weight:bold}.pdoc-code .w{color:#bbbbbb}.pdoc-code .mb{color:#666666}.pdoc-code .mf{color:#666666}.pdoc-code .mh{color:#666666}.pdoc-code .mi{color:#666666}.pdoc-code .mo{color:#666666}.pdoc-code .sa{color:#BA2121}.pdoc-code .sb{color:#BA2121}.pdoc-code .sc{color:#BA2121}.pdoc-code .dl{color:#BA2121}.pdoc-code .sd{color:#BA2121; font-style:italic}.pdoc-code .s2{color:#BA2121}.pdoc-code .se{color:#AA5D1F; font-weight:bold}.pdoc-code .sh{color:#BA2121}.pdoc-code .si{color:#A45A77; font-weight:bold}.pdoc-code .sx{color:#008000}.pdoc-code .sr{color:#A45A77}.pdoc-code .s1{color:#BA2121}.pdoc-code .ss{color:#19177C}.pdoc-code .bp{color:#008000}.pdoc-code .fm{color:#0000FF}.pdoc-code .vc{color:#19177C}.pdoc-code .vg{color:#19177C}.pdoc-code .vi{color:#19177C}.pdoc-code .vm{color:#19177C}.pdoc-code .il{color:#666666}</style><style>:root{--pdoc-background:#fff;}.pdoc{--text:#212529;--muted:#6c757d;--link:#3660a5;--link-hover:#1659c5;--code:#f8f8f8;--active:#fff598;--accent:#eee;--accent2:#c1c1c1;--nav-hover:rgba(255, 255, 255, 0.5);--name:#0066BB;--def:#008800;--annotation:#007020;}</style><style>.pdoc{color:var(--text);box-sizing:border-box;line-height:1.5;background:none;}.pdoc .pdoc-button{cursor:pointer;display:inline-block;border:solid black 1px;border-radius:2px;font-size:.75rem;padding:calc(0.5em - 1px) 1em;transition:100ms all;}.pdoc .alert{padding:1rem 1rem 1rem calc(1.5rem + 24px);border:1px solid transparent;border-radius:.25rem;background-repeat:no-repeat;background-position:.75rem center;margin-bottom:1rem;}.pdoc .alert > em{display:none;}.pdoc .alert > *:last-child{margin-bottom:0;}.pdoc .alert.note{color:#084298;background-color:#cfe2ff;border-color:#b6d4fe;background-image:url("data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A//www.w3.org/2000/svg%22%20width%3D%2224%22%20height%3D%2224%22%20fill%3D%22%23084298%22%20viewBox%3D%220%200%2016%2016%22%3E%3Cpath%20d%3D%22M8%2016A8%208%200%201%200%208%200a8%208%200%200%200%200%2016zm.93-9.412-1%204.705c-.07.34.029.533.304.533.194%200%20.487-.07.686-.246l-.088.416c-.287.346-.92.598-1.465.598-.703%200-1.002-.422-.808-1.319l.738-3.468c.064-.293.006-.399-.287-.47l-.451-.081.082-.381%202.29-.287zM8%205.5a1%201%200%201%201%200-2%201%201%200%200%201%200%202z%22/%3E%3C/svg%3E");}.pdoc .alert.tip{color:#0a3622;background-color:#d1e7dd;border-color:#a3cfbb;background-image:url("data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A//www.w3.org/2000/svg%22%20width%3D%2224%22%20height%3D%2224%22%20fill%3D%22%230a3622%22%20viewBox%3D%220%200%2016%2016%22%3E%3Cpath%20d%3D%22M2%206a6%206%200%201%201%2010.174%204.31c-.203.196-.359.4-.453.619l-.762%201.769A.5.5%200%200%201%2010.5%2013a.5.5%200%200%201%200%201%20.5.5%200%200%201%200%201l-.224.447a1%201%200%200%201-.894.553H6.618a1%201%200%200%201-.894-.553L5.5%2015a.5.5%200%200%201%200-1%20.5.5%200%200%201%200-1%20.5.5%200%200%201-.46-.302l-.761-1.77a2%202%200%200%200-.453-.618A5.98%205.98%200%200%201%202%206m6-5a5%205%200%200%200-3.479%208.592c.263.254.514.564.676.941L5.83%2012h4.342l.632-1.467c.162-.377.413-.687.676-.941A5%205%200%200%200%208%201%22/%3E%3C/svg%3E");}.pdoc .alert.important{color:#055160;background-color:#cff4fc;border-color:#9eeaf9;background-image:url("data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A//www.w3.org/2000/svg%22%20width%3D%2224%22%20height%3D%2224%22%20fill%3D%22%23055160%22%20viewBox%3D%220%200%2016%2016%22%3E%3Cpath%20d%3D%22M2%200a2%202%200%200%200-2%202v12a2%202%200%200%200%202%202h12a2%202%200%200%200%202-2V2a2%202%200%200%200-2-2zm6%204c.535%200%20.954.462.9.995l-.35%203.507a.552.552%200%200%201-1.1%200L7.1%204.995A.905.905%200%200%201%208%204m.002%206a1%201%200%201%201%200%202%201%201%200%200%201%200-2%22/%3E%3C/svg%3E");}.pdoc .alert.warning{color:#664d03;background-color:#fff3cd;border-color:#ffecb5;background-image:url("data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A//www.w3.org/2000/svg%22%20width%3D%2224%22%20height%3D%2224%22%20fill%3D%22%23664d03%22%20viewBox%3D%220%200%2016%2016%22%3E%3Cpath%20d%3D%22M8.982%201.566a1.13%201.13%200%200%200-1.96%200L.165%2013.233c-.457.778.091%201.767.98%201.767h13.713c.889%200%201.438-.99.98-1.767L8.982%201.566zM8%205c.535%200%20.954.462.9.995l-.35%203.507a.552.552%200%200%201-1.1%200L7.1%205.995A.905.905%200%200%201%208%205zm.002%206a1%201%200%201%201%200%202%201%201%200%200%201%200-2z%22/%3E%3C/svg%3E");}.pdoc .alert.caution{color:#842029;background-color:#f8d7da;border-color:#f5c2c7;background-image:url("data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A//www.w3.org/2000/svg%22%20width%3D%2224%22%20height%3D%2224%22%20fill%3D%22%23842029%22%20viewBox%3D%220%200%2016%2016%22%3E%3Cpath%20d%3D%22M11.46.146A.5.5%200%200%200%2011.107%200H4.893a.5.5%200%200%200-.353.146L.146%204.54A.5.5%200%200%200%200%204.893v6.214a.5.5%200%200%200%20.146.353l4.394%204.394a.5.5%200%200%200%20.353.146h6.214a.5.5%200%200%200%20.353-.146l4.394-4.394a.5.5%200%200%200%20.146-.353V4.893a.5.5%200%200%200-.146-.353zM8%204c.535%200%20.954.462.9.995l-.35%203.507a.552.552%200%200%201-1.1%200L7.1%204.995A.905.905%200%200%201%208%204m.002%206a1%201%200%201%201%200%202%201%201%200%200%201%200-2%22/%3E%3C/svg%3E");}.pdoc .alert.danger{color:#842029;background-color:#f8d7da;border-color:#f5c2c7;background-image:url("data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A//www.w3.org/2000/svg%22%20width%3D%2224%22%20height%3D%2224%22%20fill%3D%22%23842029%22%20viewBox%3D%220%200%2016%2016%22%3E%3Cpath%20d%3D%22M5.52.359A.5.5%200%200%201%206%200h4a.5.5%200%200%201%20.474.658L8.694%206H12.5a.5.5%200%200%201%20.395.807l-7%209a.5.5%200%200%201-.873-.454L6.823%209.5H3.5a.5.5%200%200%201-.48-.641l2.5-8.5z%22/%3E%3C/svg%3E");}.pdoc .visually-hidden{position:absolute !important;width:1px !important;height:1px !important;padding:0 !important;margin:-1px !important;overflow:hidden !important;clip:rect(0, 0, 0, 0) !important;white-space:nowrap !important;border:0 !important;}.pdoc h1, .pdoc h2, .pdoc h3{font-weight:300;margin:.3em 0;padding:.2em 0;}.pdoc > section:not(.module-info) h1{font-size:1.5rem;font-weight:500;}.pdoc > section:not(.module-info) h2{font-size:1.4rem;font-weight:500;}.pdoc > section:not(.module-info) h3{font-size:1.3rem;font-weight:500;}.pdoc > section:not(.module-info) h4{font-size:1.2rem;}.pdoc > section:not(.module-info) h5{font-size:1.1rem;}.pdoc a{text-decoration:none;color:var(--link);}.pdoc a:hover{color:var(--link-hover);}.pdoc blockquote{margin-left:2rem;}.pdoc pre{border-top:1px solid var(--accent2);border-bottom:1px solid var(--accent2);margin-top:0;margin-bottom:1em;padding:.5rem 0 .5rem .5rem;overflow-x:auto;background-color:var(--code);}.pdoc code{color:var(--text);padding:.2em .4em;margin:0;font-size:85%;background-color:var(--accent);border-radius:6px;}.pdoc a > code{color:inherit;}.pdoc pre > code{display:inline-block;font-size:inherit;background:none;border:none;padding:0;}.pdoc > section:not(.module-info){margin-bottom:1.5rem;}.pdoc .modulename{margin-top:0;font-weight:bold;}.pdoc .modulename a{color:var(--link);transition:100ms all;}.pdoc .git-button{float:right;border:solid var(--link) 1px;}.pdoc .git-button:hover{background-color:var(--link);color:var(--pdoc-background);}.view-source-toggle-state,.view-source-toggle-state ~ .pdoc-code{display:none;}.view-source-toggle-state:checked ~ .pdoc-code{display:block;}.view-source-button{display:inline-block;float:right;font-size:.75rem;line-height:1.5rem;color:var(--muted);padding:0 .4rem 0 1.3rem;cursor:pointer;text-indent:-2px;}.view-source-button > span{visibility:hidden;}.module-info .view-source-button{float:none;display:flex;justify-content:flex-end;margin:-1.2rem .4rem -.2rem 0;}.view-source-button::before{position:absolute;content:"View Source";display:list-item;list-style-type:disclosure-closed;}.view-source-toggle-state:checked ~ .attr .view-source-button::before,.view-source-toggle-state:checked ~ .view-source-button::before{list-style-type:disclosure-open;}.pdoc .docstring{margin-bottom:1.5rem;}.pdoc section:not(.module-info) .docstring{margin-left:clamp(0rem, 5vw - 2rem, 1rem);}.pdoc .docstring .pdoc-code{margin-left:1em;margin-right:1em;}.pdoc h1:target,.pdoc h2:target,.pdoc h3:target,.pdoc h4:target,.pdoc h5:target,.pdoc h6:target,.pdoc .pdoc-code > pre > span:target{background-color:var(--active);box-shadow:-1rem 0 0 0 var(--active);}.pdoc .pdoc-code > pre > span:target{display:block;}.pdoc div:target > .attr,.pdoc section:target > .attr,.pdoc dd:target > a{background-color:var(--active);}.pdoc *{scroll-margin:2rem;}.pdoc .pdoc-code .linenos{user-select:none;}.pdoc .attr:hover{filter:contrast(0.95);}.pdoc section, .pdoc .classattr{position:relative;}.pdoc .headerlink{--width:clamp(1rem, 3vw, 2rem);position:absolute;top:0;left:calc(0rem - var(--width));transition:all 100ms ease-in-out;opacity:0;}.pdoc .headerlink::before{content:"#";display:block;text-align:center;width:var(--width);height:2.3rem;line-height:2.3rem;font-size:1.5rem;}.pdoc .attr:hover ~ .headerlink,.pdoc *:target > .headerlink,.pdoc .headerlink:hover{opacity:1;}.pdoc .attr{display:block;margin:.5rem 0 .5rem;padding:.4rem .4rem .4rem 1rem;background-color:var(--accent);overflow-x:auto;}.pdoc .classattr{margin-left:2rem;}.pdoc .decorator-deprecated{color:#842029;}.pdoc .decorator-deprecated ~ span{filter:grayscale(1) opacity(0.8);}.pdoc .name{color:var(--name);font-weight:bold;}.pdoc .def{color:var(--def);font-weight:bold;}.pdoc .signature{background-color:transparent;}.pdoc .param, .pdoc .return-annotation{white-space:pre;}.pdoc .signature.multiline .param{display:block;}.pdoc .signature.condensed .param{display:inline-block;}.pdoc .annotation{color:var(--annotation);}.pdoc .view-value-toggle-state,.pdoc .view-value-toggle-state ~ .default_value{display:none;}.pdoc .view-value-toggle-state:checked ~ .default_value{display:inherit;}.pdoc .view-value-button{font-size:.5rem;vertical-align:middle;border-style:dashed;margin-top:-0.1rem;}.pdoc .view-value-button:hover{background:white;}.pdoc .view-value-button::before{content:"show";text-align:center;width:2.2em;display:inline-block;}.pdoc .view-value-toggle-state:checked ~ .view-value-button::before{content:"hide";}.pdoc .inherited{margin-left:2rem;}.pdoc .inherited dt{font-weight:700;}.pdoc .inherited dt, .pdoc .inherited dd{display:inline;margin-left:0;margin-bottom:.5rem;}.pdoc .inherited dd:not(:last-child):after{content:", ";}.pdoc .inherited .class:before{content:"class ";}.pdoc .inherited .function a:after{content:"()";}.pdoc .search-result .docstring{overflow:auto;max-height:25vh;}.pdoc .search-result.focused > .attr{background-color:var(--active);}.pdoc .attribution{margin-top:2rem;display:block;opacity:0.5;transition:all 200ms;filter:grayscale(100%);}.pdoc .attribution:hover{opacity:1;filter:grayscale(0%);}.pdoc .attribution img{margin-left:5px;height:27px;vertical-align:bottom;width:50px;transition:all 200ms;}.pdoc table{display:block;width:max-content;max-width:100%;overflow:auto;margin-bottom:1rem;}.pdoc table th{font-weight:600;}.pdoc table th, .pdoc table td{padding:6px 13px;border:1px solid var(--accent2);}</style><style>.pdoc pre,.pdoc code,.pdoc .signature{font-family:SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono","Courier New", Courier, monospace;font-size:12px;line-height:1.4;}</style>
