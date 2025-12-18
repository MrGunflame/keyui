import { invoke } from '@tauri-apps/api/core';

export class Client {
    async send(method: string, payload: any): Promise<any> {
        let resp: any = await invoke("send_msg", {
            "method": method,
            "params": payload,
        });

        return resp;
    }

    public async version(): Promise<string> {
        return await this.send("meta/version", null);
    }

    public async loadKey(content: string): Promise<ProofId> {
        return await this.send("loading/loadKey", content);
    }

    public async proofTreeRoot(proof: ProofId): Promise<TreeNodeDesc> {
        return await this.send("proofTree/root", proof);
    }

    public async goalPrint(id: NodeId, options: PrintOptions): Promise<NodeTextDesc> {
        return await this.send("goal/print", [id, options]);
    }
    public async proofTreeChildren(proof: ProofId, nodeId: TreeNodeId): Promise<TreeNodeDesc[]>{
        return await this.send("proofTree/children", [proof,nodeId]);
    }
    public async proofTreeSubtree(proof:ProofId, nodeId: TreeNodeId): Promise<TreeNodeDesc[]>{
        return await this.send("proofTree/subtree",[proof,nodeId]);
    }
}

export type ProofId = {
    env: EnvId;
    proofId: string,
};

export type EnvId = {
    envId: string,
};

export type NodeId = {
    nodeId: string;
    proofId: ProofId;
};

export type TreeNodeDesc = {
    id: NodeId;
    name: string;
};

export type PrintOptions = {
    unicode: boolean,
    width: number,
    indentation: number,
    pure: boolean,
    termLabels: boolean,
};

export type NodeTextDesc = {
    id: NodeTextId,
    result: string,
};

export type NodeTextId = {
    nodeId: NodeId,
    nodeTextId: number,
};
export type TreeNodeId ={
    id: string;
};
