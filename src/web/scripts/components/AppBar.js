customElements.define(
  "app-bar",
  class extends HTMLElement {
    constructor() {
      super();
      this.template = document.getElementById("AppBar").content;
    }
    render() {
      this.appendChild(this.template.cloneNode(true));
    }
    connectedCallback() {
      if (!this.rendered) {
        this.render();
        this.rendered = true;
      }
    }
    disconnectedCallback() {}
    static get observedAttributes() {
      return [
        /* array of attribute names to monitor for changes */
      ];
    }
    attributeChangedCallback(name, oldValue, newValue) {}
  }
);
