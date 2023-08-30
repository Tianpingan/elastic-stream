import signal
from ducktape.utils.util import wait_until
from ducktape.services.service import Service
from ducktape.cluster.remoteaccount import RemoteCommandError
class VerifiableProducer(Service):
    ROOT = "/home/ducker/verifiable_producer"

    def __init__(self, context, num_nodes, pd, start_seq, count, stream_id=-1, replica=1):
        self.pd = pd
        self.start_seq = start_seq
        self.count = count
        self.stream_id = stream_id
        self.replica = replica
        super(VerifiableProducer, self).__init__(context, num_nodes)

    def restart_cluster(self):
        for node in self.nodes:
            self.restart_node(node)

    def restart_node(self, node):
        """Restart the given node."""
        self.stop_node(node)
        self.start_node(node)

    def start_node(self, node):
        idx = self.idx(node)
        self.logger.info("Starting Metadata node %d on %s", idx, node.account.hostname)
        node.account.ssh("mkdir -p %s" % VerifiableProducer.ROOT)
        cmd = self.start_cmd(node)
        output = node.account.ssh_output(cmd, allow_fail=True).decode('utf-8')
        print (output)
        if "Append complete" in output:
            pass
        else:
            raise Exception("Test Failed")

    def start_cmd(self, node):
        cmd = "cd " + VerifiableProducer.ROOT + ";"
        cmd += "export E2E_END_POINT=" + self.pd.get_hostname() + ":12378;"
        cmd += "export E2E_KV_END_POINT=" + self.pd.get_hostname() + ":12379;"
        cmd += "export E2E_COUNT=" + str(self.count) + ";"
        cmd += "export E2E_STREAM_ID=" + str(self.stream_id) + ";"
        cmd += "export E2E_START_SEQ=" + str(self.start_seq) + ";"
        cmd += "export E2E_REPLICA=" + str(self.replica) + ";"
        cmd += "java -cp /opt/*.jar  com.automq.elasticstream.client.tools.e2e.VerifiableProducer"
        return cmd

    def pids(self, node):
        try:
            cmd = "ps -a | grep MetadataTest | awk '{print $1}'"
            pid_arr = [pid for pid in node.account.ssh_capture(cmd, allow_fail=True, callback=int)]
            return pid_arr
        except (RemoteCommandError, ValueError) as e:
            return []

    def alive(self, node):
        return len(self.pids(node)) > 0

    def kill_node(self, node):
        idx = self.idx(node)
        self.logger.info("Killing %s node %d on %s" % (type(self).__name__, idx, node.account.hostname))
        self.signal_node(node, signal.SIGKILL)
        wait_until(lambda: not self.alive(node), timeout_sec=5, err_msg="Timed out waiting for MetadataTest to be killed.")

    def stop_node(self, node):
        idx = self.idx(node)
        self.logger.info("Stopping %s node %d on %s" % (type(self).__name__, idx, node.account.hostname))
        self.signal_node(node)
        wait_until(lambda: not self.alive(node), timeout_sec=5, err_msg="Timed out waiting for MetadataTest to stop.")

    def signal_node(self, node, sig=signal.SIGTERM):
        pids = self.pids(node)
        for pid in pids:
            node.account.signal(pid, sig)

    def clean_node(self, node):
        self.stop_node(node)
        node.account.ssh("sudo rm -rf -- %s" % VerifiableProducer.ROOT, allow_fail=False)
