import React, { useMemo } from 'react';
import ReactJson from 'react18-json-view';
import 'react18-json-view/src/style.css';

interface IJsonPreviewProps {
  arrayBuffer?: ArrayBuffer;
}

export const JsonPreview: React.FC<IJsonPreviewProps> = ({ arrayBuffer }) => {
  const json = useMemo(() => {
    if (!arrayBuffer) {
      return null;
    }
    return JSON.parse(new TextDecoder().decode(arrayBuffer));
  }, [arrayBuffer]);
  if (!json) {
    return null;
  }
  return <ReactJson className="text-xs" src={json} />;
};
