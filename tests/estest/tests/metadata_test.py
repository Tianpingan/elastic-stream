import time
from ducktape.tests.test import Test
from estest.services.pd import PD
from ducktape.mark.resource import cluster
from estest.services.range_server import RangeServer
from estest.services.metadata import Metadata
from ducktape.mark import matrix
class MetadataTest(Test):
    def __init__(self, test_context):
        super(MetadataTest, self).__init__(test_context=test_context)
    @cluster(num_nodes=4)
    @matrix(count=[100, 1000])
    def test_metadata(self, count):
        pd = PD(self.test_context, num_nodes=3)
        pd.start()
        metadata = Metadata(self.test_context, num_nodes=1, pd=pd, count=count)
        metadata.start()
