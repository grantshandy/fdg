<script>
import Title from './components/Title.vue'
import ViewSwitcher from './components/ViewSwitcher.vue'

import init, { generate_svg } from 'fdg-img'

export default {
  name: 'App',

  components: {
    Title,
    ViewSwitcher,
  },

  data() {
    return {
      addNodeText: '',
      addEdgeSourceText: '',
      addEdgeTargetText: '',
      error: null,
      svg: '',
      isView: false,
      graph: {
        graph: {
          nodes: {
            B: {},
            A: {},
            C: {}
          },
          edges: [
            {
              source: 'A',
              target: 'B'
            },
            {
              source: 'B',
              target: 'C'
            },
            {
              source: 'C',
              target: 'A'
            }
          ],
        }
      },
      settings: {
        iterations: 2000,
        dt: 0.035,
        showText: true,
        textSize: 20,
        textColor: {
          r: 248,
          g: 250,
          b: 252,
          a: 1.0
        },
        backgroundColor: {
          r: 0,
          g: 0,
          b: 0,
          a: 0.0
        },
        nodeColor: {
          r: 30,
          g: 41,
          b: 59,
          a: 1.0
        },
        nodeSize: 10,
        edgeColor: {
          r: 100,
          g: 116,
          b: 139,
          a: 1.0
        },
        edgeSize: 5,
      }
    }
  },

  async mounted() {
    await init();

    // this.graph = await fetch('https://raw.githubusercontent.com/jsongraph/json-graph-specification/master/examples/les_miserables.json')
    //   .then(response => response.json());
  },

  methods: {
    setView(isView) {
      this.isView = isView;
    },
    
    async generateImage() {
      this.error = null;

      try {
        this.svg = generate_svg(this.graph, this.settings);
        document.getElementById('svgContainer').innerHTML = this.svg;
      } catch (error) {
        this.error = error;
      }

      document.getElementById('svgContainer').children[0].setAttribute('width', '100%');
    },

    removeNode(node) {
      delete this.graph.graph.nodes[node];
    },

    removeEdge(index) {
      this.graph.graph.edges.splice(index, 1);
    },

    addNode(event) {
      if (event != null && event.key != 'Enter') {
        return;
      }

      let name = this.addNodeText;

      if (name == '') {
        return;
      }

      if (this.graph.graph.nodes[name] == null || this.graph.graph.nodes[name] == undefined) {
        this.graph.graph.nodes[name] = {}
      }

      this.addNodeText = '';
    },

    addEdge() {
      let source = this.addEdgeSourceText;
      let target = this.addEdgeTargetText;

      if (source == null || target == null) {
        return;
      }

      let obj = {
        source: source,
        target: target,
      };

      if (!this.graph.graph.edges.includes(obj)) {
        this.graph.graph.edges.push(obj);

        this.addEdgeSourceText = '';
        this.addEdgeTargetText = '';
      }
    }
  }
}
</script>

<template>
  <div class="bg-slate-300 py-6 min-h-screen">
    <div class="w-5/6 md:w-1/2 mx-auto space-y-4">
      <Title class="c-block" />
      <div v-if="error" class="e-block">
        <p class="font-bold italic text-xl">{{ error }}</p>
      </div>
      <div class="bg-slate-700 text-slate-50 rounded-md shadow-lg">
        <ViewSwitcher v-on:setView="setView" />
        <div class="p-2">
          <div v-if="!isView" class="space-y-3 mx-auto text-center">
            <h3 class="italic text-2xl">Nodes</h3>
            <div class="bg-slate-800 rounded-md shadow-lg w-full md:w-2/3 mx-auto">
              <div v-for="(value, key) in graph.graph.nodes" class="p-2 space-y-1" :key="key">
                <div class="bg-slate-700 px-2 py-1 pl-3 rounded-md shadow-lg flow-root content-center">
                  <p class="float-left font-bold my-auto">{{ key }}</p>
                  <button v-on:click="removeNode(key)" type="button" class="float-right rounded-md p-2 inline-flex items-center justify-center rounded-full hover:bg-slate-600">
                    <span class="sr-only">Close</span>
                    <svg class="h-4 w-4" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor" aria-hidden="true">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                    </svg>
                  </button>
                </div>
              </div>
            </div>
            <div class="bg-slate-800 border-4 border-solid border-slate-800 rounded-md shadow-lg w-full md:w-2/3 mx-auto flex">
              <input type="text" v-model="addNodeText" v-on:keypress="addNode" class="rounded-tl-md rounded-bl-md flex-grow bg-slate-800 px-2 py-1"/>
              <button v-on:click="addNode()" type="button" class="rounded-tr-md rounded-br-md p-2 inline-flex items-center justify-center bg-slate-700 hover:bg-slate-600 font-semibold">Add</button>
            </div>

            <h3 class="italic text-2xl">Edges</h3>
            <div class="bg-slate-800 rounded-md shadow-lg w-full md:w-2/3 mx-auto">
              <div v-for="(edge, index) in graph.graph.edges" class="p-2 space-y-1" :key="edge">
                <div class="bg-slate-700 px-2 py-1 pl-3 rounded-md shadow-lg flow-root content-center">
                  <p class="float-left font-bold my-auto">Source: "{{ edge.source }}", Target: "{{ edge.target }}"</p>
                  <button v-on:click="removeEdge(index)" type="button" class="float-right rounded-md p-2 inline-flex items-center justify-center rounded-full hover:bg-slate-600">
                    <span class="sr-only">Close</span>
                    <svg class="h-4 w-4" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor" aria-hidden="true">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                    </svg>
                  </button>
                </div>
              </div>
            </div>
            <div class="bg-slate-800 border-4 border-solid border-slate-800 rounded-md shadow-lg w-full md:w-2/3 mx-auto flex">
              <div class="flex-grow grid grid-cols-4">
                <p class="bg-slate-700 align-middle">Source:</p>
                <input type="text" v-model="addEdgeSourceText" class="rounded-tl-md rounded-bl-md bg-slate-800 px-2 py-1"/>
                <p class="bg-slate-700 align-middle">Target:</p>
                <input type="text" v-model="addEdgeTargetText" class="rounded-tl-md rounded-bl-md bg-slate-800 px-2 py-1"/>
              </div>
              <button v-on:click="addEdge()" type="button" class="rounded-tr-md rounded-br-md p-2 inline-flex items-center justify-center bg-slate-700 hover:bg-slate-600 font-semibold">Add</button>
            </div>
          </div>
          <div v-else>
            <pre class="font-monospace rounded-md bg-slate-800 p-2">{{ JSON.stringify(graph, null, 2) }}</pre>
          </div>
        </div>
        <div class="flow-root pr-2 pb-2">
          <button v-if="!svg" class="float-right px-2 py-1 rounded-md shadow-lg bg-slate-600 hover:bg-slate-500" v-on:click="generateImage">Export</button>
          <button v-else class="float-right px-2 py-1 rounded-md shadow-lg bg-slate-600 hover:bg-slate-500" v-on:click="generateImage">Update</button>
        </div>
      </div>
      <div v-if="svg != '' || svg != null" class="c-block justify-items-center flex">
        <div class="mx-auto object-contain" id="svgContainer"></div>
      </div>
    </div>
  </div>
</template>
