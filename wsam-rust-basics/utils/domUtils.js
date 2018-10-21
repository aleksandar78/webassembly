export const appendStringToBody = (text) => {
    const el = document.createTextNode(text);
    document.body.appendChild(el);
};